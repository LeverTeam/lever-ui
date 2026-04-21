use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::Rect;

pub trait Widget {
    fn build(&self) -> Vec<Box<dyn Widget>> {
        Vec::new()
    }

    fn layout(&self, constraints: Constraints, children: &[LayoutNode]) -> LayoutResult;

    fn draw(&self, rect: Rect, draw_list: &mut DrawList);

    fn on_event(&self, _event: &crate::event::FrameworkEvent) {}
}
