use crate::animated::animated_spring;
use crate::animation::Spring;
use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::state::{get_or_set_state, update_state};
use crate::types::{Color, Point, Rect, Size};
use crate::widget::Widget;

#[derive(Debug, Clone, Default)]
struct SliderState {
    is_dragging: bool,
}

pub struct Slider<M> {
    pub id: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    pub disabled: bool,
    pub on_changed: Option<Box<dyn Fn(f32) -> M>>,
    pub label_formatter: Option<Box<dyn Fn(f32) -> String>>,
}

impl<M> Slider<M> {
    pub fn new(id: impl Into<String>, value: f32) -> Self {
        Self {
            id: id.into(),
            value,
            min: 0.0,
            max: 1.0,
            step: None,
            disabled: false,
            on_changed: None,
            label_formatter: None,
        }
    }

    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self.value = self.value.clamp(min, max);
        self
    }

    pub fn with_step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_label_formatter<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> String + 'static,
    {
        self.label_formatter = Some(Box::new(f));
        self
    }

    pub fn on_changed<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> M + 'static,
    {
        self.on_changed = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Slider<M> {
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
            width: 200.0,
            height: 32.0,
        });
        LayoutResult { size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        let state = get_or_set_state::<SliderState, _>(&self.id, || SliderState::default());
        let is_hovered = pointer_pos.map_or(false, |pos| rect.contains(pos)) && !self.disabled;
        let is_focused = focused_id == Some(&self.id);

        let track_height = 6.0;
        let track_rect = Rect {
            x: rect.x + 12.0,
            y: rect.y + (rect.height - track_height) / 2.0,
            width: rect.width - 24.0,
            height: track_height,
        };

        let normalized_val = if self.max > self.min {
            (self.value - self.min) / (self.max - self.min)
        } else {
            0.0
        };

        let animated_val =
            animated_spring(&format!("{}_val", self.id), normalized_val, Spring::SNAPPY);

        draw_list.rounded_rect(track_rect, theme.surface_variant, track_height / 2.0);

        let active_width = track_rect.width * animated_val;
        let active_color = if self.disabled {
            theme.text_muted.with_alpha(0.5)
        } else {
            theme.primary
        };

        draw_list.rounded_rect(
            Rect {
                x: track_rect.x,
                y: track_rect.y,
                width: active_width,
                height: track_height,
            },
            active_color,
            track_height / 2.0,
        );

        if is_focused && !self.disabled {
            draw_list.stroke_rect(
                rect.inset(-2.0, -2.0),
                theme.primary.with_alpha(0.3),
                8.0,
                2.0,
            );
        }

        let thumb_base_radius = 8.0;
        let thumb_scale = animated_spring(
            &format!("{}_thumb_scale", self.id),
            if state.is_dragging {
                1.3
            } else if is_hovered {
                1.15
            } else {
                1.0
            },
            Spring::SNAPPY,
        );

        let thumb_radius = thumb_base_radius * thumb_scale;
        let thumb_x = track_rect.x + active_width;
        let thumb_y = rect.y + rect.height / 2.0;

        let thumb_rect = Rect {
            x: thumb_x - thumb_radius,
            y: thumb_y - thumb_radius,
            width: thumb_radius * 2.0,
            height: thumb_radius * 2.0,
        };

        let thumb_color = if self.disabled {
            theme.surface_variant
        } else if state.is_dragging {
            theme.primary
        } else {
            Color::WHITE
        };

        draw_list.shadowed_rect(
            thumb_rect,
            thumb_color,
            thumb_radius,
            crate::types::BoxShadow {
                offset: Point { x: 0.0, y: 2.0 },
                blur: 6.0,
                color: theme.shadow_color,
            },
        );

        let border_color = if self.disabled {
            theme.border.with_alpha(0.5)
        } else if state.is_dragging || is_focused {
            theme.primary
        } else {
            theme.border
        };
        draw_list.stroke_rect(thumb_rect, border_color, thumb_radius, 2.0);

        if state.is_dragging {
            let label_text = if let Some(formatter) = &self.label_formatter {
                formatter(self.value)
            } else if let Some(step) = self.step {
                if step >= 1.0 {
                    format!("{:.0}", self.value)
                } else {
                    format!("{:.2}", self.value)
                }
            } else {
                format!("{:.2}", self.value)
            };

            let label_size = 14.0;
            let layout = text_system.shape(
                &label_text,
                label_size,
                theme.text,
                None,
                crate::types::TextAlign::Center,
            );

            let label_rect = Rect {
                x: thumb_x - layout.width / 2.0 - 6.0,
                y: thumb_y - thumb_radius - 30.0,
                width: layout.width + 12.0,
                height: 24.0,
            };

            draw_list.rounded_rect(label_rect, theme.surface, 6.0);
            draw_list.stroke_rect(label_rect, theme.border, 6.0, 1.0);

            draw_list.text(
                Point {
                    x: label_rect.x + 6.0,
                    y: label_rect.y + (label_rect.height - layout.height) / 2.0,
                },
                layout.glyphs,
            );
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        if self.disabled {
            return Vec::new();
        }

        let mut messages = Vec::new();
        let state = get_or_set_state::<SliderState, _>(&self.id, || SliderState::default());

        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    *focused_id = Some(self.id.clone());
                    update_state::<SliderState, _>(&self.id, |s| s.is_dragging = true);
                    *consumed = true;

                    let new_value = self.calculate_value(position.x, rect);
                    if (new_value - self.value).abs() > 0.0001 {
                        self.value = new_value;
                        if let Some(on_changed) = &self.on_changed {
                            messages.push(on_changed(self.value));
                        }
                    }
                }
            }
            FrameworkEvent::PointerMove { position } => {
                if state.is_dragging {
                    let new_value = self.calculate_value(position.x, rect);
                    if (new_value - self.value).abs() > 0.0001 {
                        self.value = new_value;
                        if let Some(on_changed) = &self.on_changed {
                            messages.push(on_changed(self.value));
                        }
                    }
                    *consumed = true;
                }
            }
            FrameworkEvent::PointerUp { .. } => {
                if state.is_dragging {
                    update_state::<SliderState, _>(&self.id, |s| s.is_dragging = false);
                    *consumed = true;
                }
            }
            FrameworkEvent::KeyDown { key, .. } => {
                if focused_id.as_deref() == Some(&self.id) {
                    let step = self.step.unwrap_or((self.max - self.min) / 20.0);
                    let mut new_value = self.value;

                    match key {
                        crate::event::Key::Right | crate::event::Key::Up => {
                            new_value = (self.value + step).min(self.max);
                        }
                        crate::event::Key::Left | crate::event::Key::Down => {
                            new_value = (self.value - step).max(self.min);
                        }
                        crate::event::Key::Home => {
                            new_value = self.min;
                        }
                        crate::event::Key::End => {
                            new_value = self.max;
                        }
                        _ => {}
                    }

                    if (new_value - self.value).abs() > 0.0001 {
                        self.value = new_value;
                        if let Some(on_changed) = &self.on_changed {
                            messages.push(on_changed(self.value));
                        }
                        *consumed = true;
                    }
                }
            }
            _ => {}
        }
        messages
    }
}

impl<M> Slider<M> {
    fn calculate_value(&self, mouse_x: f32, rect: Rect) -> f32 {
        let track_x = rect.x + 12.0;
        let track_width = rect.width - 24.0;
        let local_x = (mouse_x - track_x).clamp(0.0, track_width);
        let normalized = local_x / track_width;

        let mut value = self.min + normalized * (self.max - self.min);

        if let Some(step) = self.step {
            value = (value / step).round() * step;
        }

        value.clamp(self.min, self.max)
    }
}
