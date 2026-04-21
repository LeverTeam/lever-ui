use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin_winit::{DisplayBuilder, GlWindow};
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::config::AppConfig;
use crate::context::GlContext;
use lever_core::draw::DrawList;
use lever_core::types::Size;
use lever_renderer::Renderer;

pub struct Application {
    config: AppConfig,
    build_ui: Box<dyn Fn(&mut DrawList)>,
}

impl Application {
    pub fn new(config: AppConfig, build_ui: Box<dyn Fn(&mut DrawList)>) -> Self {
        Self { config, build_ui }
    }

    pub fn run(self) {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        let mut handler = AppHandler::new(self.config, self.build_ui);
        event_loop.run_app(&mut handler).expect("Failed to run app");
    }
}

struct AppHandler {
    config: AppConfig,
    build_ui: Box<dyn Fn(&mut DrawList)>,

    window: Option<Arc<Window>>,
    gl_context: Option<GlContext>,
    renderer: Option<Renderer>,
    draw_list: DrawList,
}

impl AppHandler {
    fn new(config: AppConfig, build_ui: Box<dyn Fn(&mut DrawList)>) -> Self {
        Self {
            config,
            build_ui,
            window: None,
            gl_context: None,
            renderer: None,
            draw_list: DrawList::new(),
        }
    }
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_attributes = WindowAttributes::default()
            .with_title(&self.config.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.config.width,
                self.config.height,
            ));

        let template_builder = glutin::config::ConfigTemplateBuilder::new().with_alpha_size(8);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, gl_config) = display_builder
            .build(event_loop, template_builder, |configs| {
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
            .expect("Failed to create window and GL config");

        let window = Arc::new(window.expect("Failed to create window"));

        let raw_window_handle = window
            .window_handle()
            .expect("Failed to get window handle")
            .as_raw();

        let gl_display = gl_config.display();

        let context_attributes =
            glutin::context::ContextAttributesBuilder::new().build(Some(raw_window_handle));

        let fallback_context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("Failed to create GL context")
                })
        };

        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");
        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = gl_context.make_current(&gl_surface).unwrap();

        let glow_context = unsafe {
            glow::Context::from_loader_function(|s| {
                gl_display.get_proc_address(&std::ffi::CString::new(s).unwrap())
            })
        };

        let renderer = Renderer::new(Arc::new(glow_context)).expect("Failed to create renderer");

        self.window = Some(window);
        self.gl_context = Some(GlContext {
            context: gl_context,
            surface: gl_surface,
            display: gl_display,
        });
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
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

                    renderer.begin_frame(viewport, self.config.clear_color);
                    self.draw_list.clear();
                    (self.build_ui)(&mut self.draw_list);
                    renderer.render(&self.draw_list);
                    renderer.end_frame();

                    gl_context.swap_buffers().expect("Failed to swap buffers");
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
