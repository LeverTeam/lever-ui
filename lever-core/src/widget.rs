use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::Rect;

pub trait Widget {
    fn build(&self) -> Vec<Box<dyn Widget>> {
        Vec::new()
    }

    fn layout(
        &self,
        constraints: Constraints,
        children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult;

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    );

    fn on_event(
        &mut self,
        _event: &crate::event::FrameworkEvent,
        _rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> bool {
        false
    }

    fn flex(&self) -> u32 {
        0
    }
}
