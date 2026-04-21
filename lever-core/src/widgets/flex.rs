use crate::draw::DrawList;
use crate::layout::{Constraints, FlexDirection, FlexLayout, LayoutNode, LayoutResult};
use crate::types::Rect;
use crate::widget::Widget;

pub struct Flex {
    pub direction: FlexDirection,
    pub children: Vec<Box<dyn Widget>>,
}

impl Flex {
    pub fn row(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            direction: FlexDirection::Row,
            children,
        }
    }

    pub fn column(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            direction: FlexDirection::Column,
            children,
        }
    }
}

impl Widget for Flex {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
    ) -> LayoutResult {
        let solver = FlexLayout::new(self.direction);
        let (result, _) = solver.layout(constraints, &self.children, text_system);
        result
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
    ) {
        let solver = FlexLayout::new(self.direction);
        let (_result, child_rects) = solver.layout(
            Constraints::tight(rect.width, rect.height),
            &self.children,
            text_system,
        );

        for (i, child) in self.children.iter().enumerate() {
            let mut child_rect = child_rects[i];
            child_rect.x += rect.x;
            child_rect.y += rect.y;
            child.draw(child_rect, draw_list, text_system);
        }
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
    ) -> bool {
        let solver = FlexLayout::new(self.direction);
        let (_result, child_rects) = solver.layout(
            Constraints::tight(rect.width, rect.height),
            &self.children,
            text_system,
        );

        for (i, child) in self.children.iter_mut().enumerate() {
            let mut child_rect = child_rects[i];
            child_rect.x += rect.x;
            child_rect.y += rect.y;

            if child.on_event(event, child_rect, text_system) {
                return true;
            }
        }
        false
    }
}
