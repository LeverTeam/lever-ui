use crate::animated::{animated_color, animated_spring};
use crate::animation::Spring;
use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Point, Rect, Size};
use crate::widget::Widget;

pub struct Checkbox<M> {
    pub id: String,
    pub is_checked: bool,
    pub label: Option<String>,
    pub on_changed: Option<Box<dyn Fn(bool) -> M>>,
}

impl<M> Checkbox<M> {
    pub fn new(id: impl Into<String>, is_checked: bool) -> Self {
        Self {
            id: id.into(),
            is_checked,
            label: None,
            on_changed: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn on_changed<F>(mut self, f: F) -> Self
    where
        F: Fn(bool) -> M + 'static,
    {
        self.on_changed = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Checkbox<M> {
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
            let layout = text_system.shape(label, 14.0, Color::WHITE, None);
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

        let box_rect = Rect {
            x: rect.x,
            y: rect.y,
            width: 24.0,
            height: 24.0,
        };

        // Animate background color
        let target_bg = if self.is_checked {
            theme.primary
        } else if is_hovered {
            theme.surface_variant
        } else {
            Color::rgba(0.0, 0.0, 0.0, 0.0)
        };
        let bg_color = animated_color(&format!("{}_bg", self.id), target_bg, 0.15);

        // Animate border color
        let target_border = if self.is_checked {
            theme.primary
        } else if is_hovered {
            theme.text_muted
        } else {
            theme.border
        };
        let border_color = animated_color(&format!("{}_border", self.id), target_border, 0.15);

        // Draw box
        draw_list.rounded_rect(box_rect, bg_color, theme.radius_sm);
        draw_list.stroke_rect(box_rect, border_color, theme.radius_sm, 2.0);

        // Draw checkmark with spring animation
        let check_scale = animated_spring(
            &format!("{}_check_scale", self.id),
            if self.is_checked { 1.0 } else { 0.0 },
            Spring::SNAPPY,
        );

        if check_scale > 0.01 {
            let check_size = 12.0 * check_scale;
            let check_rect = Rect {
                x: box_rect.x + (box_rect.width - check_size) / 2.0,
                y: box_rect.y + (box_rect.height - check_size) / 2.0,
                width: check_size,
                height: check_size,
            };

            draw_list.rounded_rect(check_rect, theme.on_primary, 2.0);
        }

        if let Some(label) = &self.label {
            let layout = text_system.shape(label, 14.0, theme.text, None);
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
                    self.is_checked = !self.is_checked;
                    if let Some(on_changed) = &self.on_changed {
                        messages.push(on_changed(self.is_checked));
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
