use crate::animated::{animated_color, animated_spring};
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
    pub value: f32, // 0.0 to 1.0
    pub on_changed: Option<Box<dyn Fn(f32) -> M>>,
}

impl<M> Slider<M> {
    pub fn new(id: impl Into<String>, value: f32) -> Self {
        Self {
            id: id.into(),
            value: value.clamp(0.0, 1.0),
            on_changed: None,
        }
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
        _text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        let state = get_or_set_state::<SliderState, _>(&self.id, || SliderState::default());
        let is_hovered = pointer_pos.map_or(false, |pos| rect.contains(pos));

        let track_height = 6.0;
        let track_rect = Rect {
            x: rect.x + 12.0,
            y: rect.y + (rect.height - track_height) / 2.0,
            width: rect.width - 24.0,
            height: track_height,
        };

        // Animate the value to make it feel smooth when clicking
        let animated_val = animated_spring(&format!("{}_val", self.id), self.value, Spring::SNAPPY);

        // Track background
        draw_list.rounded_rect(track_rect, theme.surface_variant, track_height / 2.0);

        // Active track
        let active_width = track_rect.width * animated_val;
        draw_list.rounded_rect(
            Rect {
                x: track_rect.x,
                y: track_rect.y,
                width: active_width,
                height: track_height,
            },
            theme.primary,
            track_height / 2.0,
        );

        // Thumb
        let thumb_base_radius = 8.0;
        // Animate thumb size on hover/drag
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

        // Animate thumb color
        let target_thumb_color = if state.is_dragging {
            theme.primary
        } else {
            Color::WHITE
        };
        let thumb_color =
            animated_color(&format!("{}_thumb_color", self.id), target_thumb_color, 0.1);

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

        // Thumb border
        let border_color = if state.is_dragging {
            theme.primary
        } else {
            theme.border
        };
        draw_list.stroke_rect(thumb_rect, border_color, thumb_radius, 2.0);
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        let state = get_or_set_state::<SliderState, _>(&self.id, || SliderState::default());

        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    *focused_id = Some(self.id.clone());
                    update_state::<SliderState, _>(&self.id, |s| s.is_dragging = true);

                    let new_value = self.calculate_value(position.x, rect);
                    if (new_value - self.value).abs() > 0.001 {
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
                    if (new_value - self.value).abs() > 0.001 {
                        self.value = new_value;
                        if let Some(on_changed) = &self.on_changed {
                            messages.push(on_changed(self.value));
                        }
                    }
                }
            }
            FrameworkEvent::PointerUp { .. } => {
                if state.is_dragging {
                    update_state::<SliderState, _>(&self.id, |s| s.is_dragging = false);
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
        local_x / track_width
    }
}
