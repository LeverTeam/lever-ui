use crate::animated::{animated_color, animated_spring};
use crate::animation::Spring;
use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Point, Rect, Size};
use crate::widget::Widget;

pub struct RadioButton<M> {
    pub id: String,
    pub is_selected: bool,
    pub label: Option<String>,
    pub on_selected: Option<Box<dyn Fn() -> M>>,
}

impl<M> RadioButton<M> {
    pub fn new(id: impl Into<String>, is_selected: bool) -> Self {
        Self {
            id: id.into(),
            is_selected,
            label: None,
            on_selected: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn on_selected<F>(mut self, f: F) -> Self
    where
        F: Fn() -> M + 'static,
    {
        self.on_selected = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for RadioButton<M> {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let mut width = 24.0;
        if let Some(label) = &self.label {
            let layout = text_system.shape(label, 14.0, Color::WHITE);
            width += 12.0 + layout.width;
        }

        let size = constraints.clamp_size(Size {
            width,
            height: 24.0,
        });
        LayoutResult { size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        let is_hovered = pointer_pos.map_or(false, |pos| rect.contains(pos));

        let circle_radius = 12.0;
        let circle_center = Point {
            x: rect.x + circle_radius,
            y: rect.y + circle_radius,
        };
        let circle_rect = Rect {
            x: rect.x,
            y: rect.y,
            width: 24.0,
            height: 24.0,
        };

        // Animate border color
        let target_border = if self.is_selected {
            theme.primary
        } else if is_hovered {
            theme.text_muted
        } else {
            theme.border
        };
        let border_color = animated_color(&format!("{}_border", self.id), target_border, 0.15);

        // Draw outer circle
        draw_list.stroke_rect(circle_rect, border_color, circle_radius, 2.0);

        // Draw inner dot with spring animation
        let dot_scale = animated_spring(
            &format!("{}_dot_scale", self.id),
            if self.is_selected { 1.0 } else { 0.0 },
            Spring::SNAPPY,
        );

        if dot_scale > 0.01 {
            let dot_radius = 6.0 * dot_scale;
            let dot_rect = Rect {
                x: circle_center.x - dot_radius,
                y: circle_center.y - dot_radius,
                width: dot_radius * 2.0,
                height: dot_radius * 2.0,
            };

            draw_list.rounded_rect(dot_rect, theme.primary, dot_radius);
        }

        if let Some(label) = &self.label {
            let layout = text_system.shape(label, 14.0, theme.text);
            draw_list.text(
                Point {
                    x: (rect.x + 36.0).round(),
                    y: (rect.y + (rect.height - layout.height) / 2.0).round(),
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
        _focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    if !self.is_selected {
                        if let Some(on_selected) = &self.on_selected {
                            messages.push(on_selected());
                        }
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
