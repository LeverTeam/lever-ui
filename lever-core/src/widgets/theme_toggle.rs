use crate::animation::AnimationController;
use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::state::get_or_set_state;
use crate::theme::ThemeMode;
use crate::types::{Color, Rect, Size};
use crate::widget::Widget;

pub struct ThemeToggle<M> {
    pub id: String,
    pub current_mode: ThemeMode,
    pub on_changed: Option<Box<dyn Fn(ThemeMode) -> M>>,
}

impl<M> ThemeToggle<M> {
    pub fn new(id: impl Into<String>, current_mode: ThemeMode) -> Self {
        Self {
            id: id.into(),
            current_mode,
            on_changed: None,
        }
    }

    pub fn on_changed<F>(mut self, f: F) -> Self
    where
        F: Fn(ThemeMode) -> M + 'static,
    {
        self.on_changed = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for ThemeToggle<M> {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let size = constraints.clamp_size(Size {
            width: 40.0,
            height: 40.0,
        });
        LayoutResult { size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        _text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        _pointer_pos: Option<crate::types::Point>,
    ) {
        let anim = get_or_set_state(&self.id, || {
            AnimationController::new(if self.current_mode == ThemeMode::Light {
                1.0
            } else {
                0.0
            })
        });
        let t = anim.value();

        let center_x = rect.x + rect.width / 2.0;
        let center_y = rect.y + rect.height / 2.0;

        // Background circle
        draw_list.rounded_rect(rect, theme.surface_variant, rect.width / 2.0);

        // Sun/Moon body
        let body_radius = 8.0 + (1.0 - t) * 2.0; // Slightly larger for moon
        let body_color = Color::lerp(Color::rgb(0.9, 0.9, 1.0), Color::rgb(1.0, 0.8, 0.2), t);

        draw_list.rounded_rect(
            Rect {
                x: center_x - body_radius,
                y: center_y - body_radius,
                width: body_radius * 2.0,
                height: body_radius * 2.0,
            },
            body_color,
            body_radius,
        );

        // Moon cutout (only visible when t < 1.0)
        if t < 0.99 {
            let cutout_x = center_x + 6.0 * (1.0 - t);
            let cutout_y = center_y - 4.0 * (1.0 - t);
            let cutout_radius = body_radius;

            draw_list.rounded_rect(
                Rect {
                    x: cutout_x - cutout_radius,
                    y: cutout_y - cutout_radius,
                    width: cutout_radius * 2.0,
                    height: cutout_radius * 2.0,
                },
                theme.surface_variant,
                cutout_radius,
            );
        }

        // Sun rays (only visible when t > 0.0)
        if t > 0.01 {
            let ray_count = 8;
            let ray_length = 4.0 * t;
            let inner_radius = body_radius + 4.0;

            for i in 0..ray_count {
                let angle = (i as f32 / ray_count as f32) * 2.0 * std::f32::consts::PI;
                let start_x = center_x + angle.cos() * inner_radius;
                let start_y = center_y + angle.sin() * inner_radius;
                let end_x = center_x + angle.cos() * (inner_radius + ray_length);
                let end_y = center_y + angle.sin() * (inner_radius + ray_length);

                draw_list.rounded_rect(
                    Rect {
                        x: (start_x + end_x) / 2.0 - 1.0,
                        y: (start_y + end_y) / 2.0 - 1.0,
                        width: 2.0,
                        height: 2.0,
                    },
                    body_color,
                    1.0,
                );
            }
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    *consumed = true;
                    let new_mode = match self.current_mode {
                        ThemeMode::Dark => ThemeMode::Light,
                        ThemeMode::Light => ThemeMode::Dark,
                    };

                    crate::state::update_state::<AnimationController, _>(&self.id, |anim| {
                        anim.set_target(if new_mode == ThemeMode::Light {
                            1.0
                        } else {
                            0.0
                        });
                    });

                    if let Some(on_changed) = &self.on_changed {
                        messages.push(on_changed(new_mode));
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
