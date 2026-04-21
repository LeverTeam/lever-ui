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
    ) -> LayoutResult;

    fn draw(&self, rect: Rect, draw_list: &mut DrawList, text_system: &mut crate::text::TextSystem);

    fn on_event(&self, _event: &crate::event::FrameworkEvent) {}
}
