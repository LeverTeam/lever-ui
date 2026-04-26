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
            let layout = text_system.shape(label, 14.0, Color::rgb(1.0, 1.0, 1.0));
            width += 8.0 + layout.width;
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
        _pointer_pos: Option<crate::types::Point>,
    ) {
        let box_rect = Rect {
            x: rect.x,
            y: rect.y,
            width: 24.0,
            height: 24.0,
        };

        if self.is_checked {
            draw_list.rounded_rect(box_rect, theme.primary, theme.radius_sm);
            // Draw a simple checkmark
            draw_list.rounded_rect(
                Rect {
                    x: box_rect.x + 6.0,
                    y: box_rect.y + 6.0,
                    width: 12.0,
                    height: 12.0,
                },
                theme.on_primary,
                2.0,
            );
        } else {
            draw_list.stroke_rect(box_rect, theme.border, theme.radius_sm, 2.0);
        }

        if let Some(label) = &self.label {
            let layout = text_system.shape(label, 14.0, theme.text);
            draw_list.text(
                Point {
                    x: rect.x + 32.0,
                    y: rect.y + (rect.height - layout.height) / 2.0,
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
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
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
