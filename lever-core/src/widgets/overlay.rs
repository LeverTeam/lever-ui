use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, Size};
use crate::widget::Widget;

pub struct Overlay<M> {
    pub color: Color,
    pub on_dismiss: Option<Box<dyn Fn() -> M>>,
}

impl<M> Overlay<M> {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            on_dismiss: None,
        }
    }

    pub fn on_dismiss<F: Fn() -> M + 'static>(mut self, f: F) -> Self {
        self.on_dismiss = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Overlay<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        // Overlay always stretches to fill all available space
        LayoutResult {
            size: Size {
                width: constraints.max_width,
                height: constraints.max_height,
            },
        }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        _pointer_pos: Option<crate::types::Point>,
    ) {
        draw_list.colored_rect(rect, self.color, 0.0);
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
    ) -> Vec<M> {
        match event {
            FrameworkEvent::PointerDown { position, .. } => {
                if rect.contains(*position) {
                    if let Some(on_dismiss) = &self.on_dismiss {
                        return vec![on_dismiss()];
                    }
                    return vec![];
                }
            }
            FrameworkEvent::PointerUp { position, .. }
            | FrameworkEvent::PointerMove { position, .. }
            | FrameworkEvent::Scroll { position, .. } => {
                if rect.contains(*position) {
                    return vec![];
                }
            }
            _ => {}
        }
        vec![]
    }
}
