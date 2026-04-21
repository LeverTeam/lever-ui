use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, Size};
use crate::widget::Widget;

pub struct Button<M> {
    pub label: String,
    pub color: Color,
    pub on_click: Option<Box<dyn Fn() -> M>>,
}

impl<M> Button<M> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: Color::rgb(0.2, 0.4, 0.8),
            on_click: None,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() -> M + 'static,
    {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Button<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let size = constraints.clamp_size(Size {
            width: 100.0,
            height: 36.0,
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
    ) {
        draw_list.rounded_rect(rect, self.color, theme.radius_sm);

        let layout = text_system.shape(&self.label, 14.0, Color::rgb(1.0, 1.0, 1.0));
        let x_offset = (rect.width - layout.width) / 2.0;
        let y_offset = (rect.height - layout.height) / 2.0;

        draw_list.text(
            crate::types::Point {
                x: rect.x + x_offset,
                y: rect.y + y_offset,
            },
            layout.glyphs,
        );
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            crate::event::FrameworkEvent::PointerDown { position, .. } => {
                if rect.contains(*position) {
                    if let Some(on_click) = &self.on_click {
                        messages.push(on_click());
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
