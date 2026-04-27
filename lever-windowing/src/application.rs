use crate::config::AppConfig;
use crate::context::GlContext;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::ContextAttributesBuilder;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::SurfaceAttributesBuilder;
use glutin_winit::{DisplayBuilder, GlWindow};
use lever_core::animation::AnimationController;
use lever_core::app::{App, UpdateContext};
use lever_core::draw::DrawList;
use lever_core::event::{FrameworkEvent, Modifiers, PointerButton};
use lever_core::layout::Constraints;
use lever_core::theme::Theme;
use lever_core::types::{Rect, Size};
use lever_renderer::Renderer;
use raw_window_handle::HasWindowHandle;
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes};

pub struct Application<A: App> {
    config: AppConfig,
    app: A,
}

impl<A: App> Application<A> {
    pub fn new(config: AppConfig, app: A) -> Self {
        Self { config, app }
    }

    pub fn run(self) {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        let mut handler = AppHandler::new(self.config, self.app);
        let _ = event_loop.run_app(&mut handler);
    }
}

struct AppHandler<A: App> {
    config: AppConfig,
    app: A,
    window: Option<Window>,
    gl_context: Option<GlContext>,
    renderer: Option<Renderer>,
    draw_list: DrawList,
    cursor_pos: lever_core::types::Point,
    text_system: lever_core::text::TextSystem,

    // Theme system with animation support
    theme: Theme,
    base_theme: Theme,
    target_theme: Theme,
    theme_animation: AnimationController,

    focused_id: Option<String>,
    last_frame: std::time::Instant,
    modifiers: Modifiers,
}

impl<A: App> AppHandler<A> {
    fn new(config: AppConfig, app: A) -> Self {
        let dark = Theme::dark();
        Self {
            config,
            app,
            window: None,
            gl_context: None,
            renderer: None,
            draw_list: DrawList::new(),
            cursor_pos: lever_core::types::Point { x: 0.0, y: 0.0 },
            text_system: lever_core::text::TextSystem::new(),
            theme: dark.clone(),
            base_theme: dark.clone(),
            target_theme: dark,
            theme_animation: AnimationController::new(1.0),
            focused_id: None,
            last_frame: std::time::Instant::now(),
            modifiers: Modifiers::default(),
        }
    }

    fn dispatch_event(&mut self, event: FrameworkEvent) {
        if let Some(_window) = &self.window {
            let size = _window.inner_size();
            let rect = Rect {
                x: 0.0,
                y: 0.0,
                width: size.width as f32,
                height: size.height as f32,
            };

            let mut ctx = UpdateContext::new();
            let mut view = self.app.view();

            let messages = view.on_event(
                &event,
                rect,
                &mut self.text_system,
                &self.theme,
                &mut self.focused_id,
            );

            for msg in messages {
                self.app.update(msg, &mut ctx);
            }

            if let Some(mode) = ctx.theme_mode {
                let new_theme = Theme::for_mode(mode);
                if !self.theme_animation.is_animating() {
                    self.base_theme = self.theme.clone();
                } else {
                    // If already animating, use current theme as new base
                    self.base_theme = self.theme.clone();
                }
                self.target_theme = new_theme;
                self.theme_animation.reset(0.0);
                self.theme_animation.set_target(1.0);
            }
        }
    }
}

impl<A: App> ApplicationHandler for AppHandler<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_attributes = WindowAttributes::default()
            .with_title(&self.config.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.config.width as f32,
                self.config.height as f32,
            ));

        let template = ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.expect("Failed to create window");
        let gl_display = gl_config.display();
        let raw_window_handle = window.window_handle().unwrap().as_raw();

        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let mut not_current_gl_context = Some(unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        });

        let attrs = window
            .build_surface_attributes(SurfaceAttributesBuilder::default())
            .unwrap();
        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .expect("Failed to create surface")
        };

        let gl_context = not_current_gl_context
            .take()
            .unwrap()
            .make_current(&gl_surface)
            .expect("Failed to make context current");

        // Enable V-Sync
        let _ = gl_surface.set_swap_interval(
            &gl_context,
            glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(1).unwrap()),
        );

        let glow_context = unsafe {
            Arc::new(glow::Context::from_loader_function(|s| {
                let c_str = std::ffi::CString::new(s).unwrap();
                gl_display.get_proc_address(&c_str) as *const _
            }))
        };

        let mut renderer = Renderer::new(glow_context, self.text_system.fonts())
            .expect("Failed to create renderer");

        // Initialize application state and load assets
        {
            let mut ctx = lever_core::app::Context::new(&mut renderer);
            self.app.init(&mut ctx);
        }

        let gl_context_wrapper = GlContext {
            context: gl_context,
            surface: gl_surface,
            display: gl_display,
        };

        let size = window.inner_size();
        if size.width > 0 && size.height > 0 {
            gl_context_wrapper.resize(
                NonZeroU32::new(size.width).unwrap(),
                NonZeroU32::new(size.height).unwrap(),
            );
        }

        self.gl_context = Some(gl_context_wrapper);
        self.window = Some(window);
        self.renderer = Some(renderer);

        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                _event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let (Some(gl_context), Some(window)) = (&self.gl_context, &self.window) {
                    if size.width > 0 && size.height > 0 {
                        gl_context.resize(
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        );
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(renderer), Some(gl_context), Some(window)) =
                    (&mut self.renderer, &self.gl_context, &self.window)
                {
                    let size = window.inner_size();
                    let viewport = Size {
                        width: size.width as f32,
                        height: size.height as f32,
                    };

                    renderer.begin_frame(viewport, self.theme.background);
                    self.draw_list.clear();

                    let root_widget = self.app.view();
                    let constraints = Constraints::tight(viewport.width, viewport.height);
                    let _res =
                        root_widget.layout(constraints, &[], &mut self.text_system, &self.theme);

                    root_widget.draw(
                        Rect {
                            x: 0.0,
                            y: 0.0,
                            width: viewport.width,
                            height: viewport.height,
                        },
                        &mut self.draw_list,
                        &mut self.text_system,
                        &self.theme,
                        self.focused_id.as_deref(),
                        Some(self.cursor_pos),
                    );

                    renderer.render(&self.draw_list);
                    renderer.end_frame();

                    gl_context.swap_buffers().expect("Failed to swap buffers");
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = lever_core::types::Point {
                    x: position.x as f32,
                    y: position.y as f32,
                };
                self.dispatch_event(FrameworkEvent::PointerMove {
                    position: self.cursor_pos,
                });
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let button = match button {
                    winit::event::MouseButton::Left => PointerButton::Primary,
                    winit::event::MouseButton::Right => PointerButton::Secondary,
                    winit::event::MouseButton::Middle => PointerButton::Middle,
                    _ => return,
                };

                let event = match state {
                    winit::event::ElementState::Pressed => FrameworkEvent::PointerDown {
                        position: self.cursor_pos,
                        button,
                    },
                    winit::event::ElementState::Released => FrameworkEvent::PointerUp {
                        position: self.cursor_pos,
                        button,
                    },
                };

                self.dispatch_event(event);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => lever_core::types::Point {
                        x: x * 60.0,
                        y: -y * 60.0,
                    },
                    winit::event::MouseScrollDelta::PixelDelta(pos) => lever_core::types::Point {
                        x: pos.x as f32,
                        y: pos.y as f32,
                    },
                };
                self.dispatch_event(FrameworkEvent::Scroll {
                    position: self.cursor_pos,
                    delta,
                });
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = Modifiers {
                    shift: modifiers.state().shift_key(),
                    ctrl: modifiers.state().control_key(),
                    alt: modifiers.state().alt_key(),
                    logo: modifiers.state().super_key(),
                };
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    if let Some(text) = event.text.as_ref() {
                        self.dispatch_event(FrameworkEvent::TextInput {
                            text: text.to_string(),
                        });
                    }

                    // Map winit key to lever_core key
                    use winit::keyboard::{KeyCode, PhysicalKey};
                    let key = match event.physical_key {
                        PhysicalKey::Code(KeyCode::Backspace) => {
                            Some(lever_core::event::Key::Backspace)
                        }
                        PhysicalKey::Code(KeyCode::Delete) => Some(lever_core::event::Key::Delete),
                        PhysicalKey::Code(KeyCode::Enter) => Some(lever_core::event::Key::Enter),
                        PhysicalKey::Code(KeyCode::Escape) => Some(lever_core::event::Key::Escape),
                        PhysicalKey::Code(KeyCode::Tab) => Some(lever_core::event::Key::Tab),
                        PhysicalKey::Code(KeyCode::Space) => Some(lever_core::event::Key::Space),
                        PhysicalKey::Code(KeyCode::ArrowLeft) => Some(lever_core::event::Key::Left),
                        PhysicalKey::Code(KeyCode::ArrowRight) => {
                            Some(lever_core::event::Key::Right)
                        }
                        PhysicalKey::Code(KeyCode::ArrowUp) => Some(lever_core::event::Key::Up),
                        PhysicalKey::Code(KeyCode::ArrowDown) => Some(lever_core::event::Key::Down),
                        PhysicalKey::Code(KeyCode::Home) => Some(lever_core::event::Key::Home),
                        PhysicalKey::Code(KeyCode::End) => Some(lever_core::event::Key::End),
                        _ => None,
                    };

                    if let Some(key) = key {
                        self.dispatch_event(FrameworkEvent::KeyDown {
                            key,
                            modifiers: self.modifiers,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let now = std::time::Instant::now();
        let mut dt = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        // Cap dt to avoid massive jumps (e.g. during resize or pause)
        if dt > 0.05 {
            dt = 0.05;
        }

        // Tick application and framework animations
        self.app.tick(dt);
        lever_core::state::tick_animations(dt);

        // Theme transition animation
        if self.theme_animation.is_animating() {
            self.theme_animation.tick(dt);
            self.theme = Theme::lerp(
                &self.base_theme,
                &self.target_theme,
                self.theme_animation.value(),
            );
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
