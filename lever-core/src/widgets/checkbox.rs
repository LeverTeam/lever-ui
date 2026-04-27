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
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let mut width = 24.0;
        if let Some(label) = &self.label {
            let layout =
                text_system.shape(label, 14.0, theme.text, None, crate::types::TextAlign::Left);
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
            Color::rgba(theme.primary.r, theme.primary.g, theme.primary.b, 0.1)
        } else {
            Color::TRANSPARENT
        };
        let bg_color = animated_color(&format!("{}_bg", self.id), target_bg, 0.2);

        // Animate border color
        let target_border = if self.is_checked {
            theme.primary
        } else if is_hovered {
            theme.primary
        } else {
            theme.border
        };
        let border_color = animated_color(&format!("{}_border", self.id), target_border, 0.2);

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
            let check_size = 14.0;
            let cx = box_rect.x + (box_rect.width - check_size) / 2.0;
            let cy = box_rect.y + (box_rect.height - check_size) / 2.0;

            // Draw a proper "V" checkmark using triangles for line segments
            // Segment 1: (3, 7) to (6, 10)
            // Segment 2: (6, 10) to (11, 4)
            // Coordinates are relative to the 14x14 check area

            draw_list.push_scale(
                check_scale,
                Point {
                    x: cx + 7.0,
                    y: cy + 7.0,
                },
            );

            // Draw a proper "V" checkmark using the new line primitive
            let p1 = Point {
                x: cx + 3.5,
                y: cy + 7.5,
            };
            let p2 = Point {
                x: cx + 6.5,
                y: cy + 10.5,
            };
            let p3 = Point {
                x: cx + 11.5,
                y: cy + 4.5,
            };

            let thickness = 2.5;
            draw_list.line(p1, p2, thickness, theme.on_primary);
            draw_list.line(p2, p3, thickness, theme.on_primary);

            draw_list.pop_scale();
        }

        if let Some(label) = &self.label {
            let layout =
                text_system.shape(label, 14.0, theme.text, None, crate::types::TextAlign::Left);
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
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    *consumed = true;
                    *focused_id = Some(self.id.clone());
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
