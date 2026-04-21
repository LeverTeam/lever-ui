use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutResult};
use crate::types::Rect;

pub trait Widget {
    fn layout(&self, constraints: Constraints) -> LayoutResult;
    fn draw(&self, rect: Rect, draw_list: &mut DrawList);
}
