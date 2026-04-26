use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::state::{get_or_set_state, update_state};
use crate::types::{Color, Rect, Size};
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
        _pointer_pos: Option<crate::types::Point>,
    ) {
        let track_height = 6.0;
        let track_rect = Rect {
            x: rect.x + 8.0,
            y: rect.y + (rect.height - track_height) / 2.0,
            width: rect.width - 16.0,
            height: track_height,
        };

        // Track background
        draw_list.rounded_rect(track_rect, theme.surface_variant, track_height / 2.0);

        // Active track
        let active_width = track_rect.width * self.value;
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
        let thumb_radius = 8.0;
        let thumb_x = track_rect.x + active_width;
        let thumb_y = rect.y + rect.height / 2.0;

        let thumb_rect = Rect {
            x: thumb_x - thumb_radius,
            y: thumb_y - thumb_radius,
            width: thumb_radius * 2.0,
            height: thumb_radius * 2.0,
        };

        draw_list.rounded_rect(thumb_rect, Color::rgb(1.0, 1.0, 1.0), thumb_radius);

        // Thumb border
        draw_list.stroke_rect(thumb_rect, theme.primary, thumb_radius, 2.0);
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
                update_state::<SliderState, _>(&self.id, |s| s.is_dragging = false);
            }
            _ => {}
        }
        messages
    }
}

impl<M> Slider<M> {
    fn calculate_value(&self, mouse_x: f32, rect: Rect) -> f32 {
        let track_x = rect.x + 8.0;
        let track_width = rect.width - 16.0;
        let local_x = (mouse_x - track_x).clamp(0.0, track_width);
        local_x / track_width
    }
}
