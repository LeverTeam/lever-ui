// use crate::event::FrameworkEvent;
use crate::layout::LayoutNode;
use crate::types::Point;

pub struct EventRouter;

impl EventRouter {
    pub fn hit_test<'a>(node: &'a LayoutNode, point: Point) -> Option<&'a LayoutNode> {
        for child in node.children.iter().rev() {
            if let Some(hit) = Self::hit_test(child, point) {
                return Some(hit);
            }
        }

        if node.rect.contains(point) {
            return Some(node);
        }

        None
    }
}
