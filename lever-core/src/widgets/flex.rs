use crate::draw::DrawList;
use crate::layout::{Constraints, FlexDirection, FlexLayout, LayoutNode, LayoutResult};
use crate::types::Rect;
use crate::widget::Widget;

pub struct Flex<M> {
    pub direction: FlexDirection,
    pub children: Vec<Box<dyn Widget<M>>>,
    pub gap: f32,
}

impl<M> Flex<M> {
    pub fn row(children: Vec<Box<dyn Widget<M>>>) -> Self {
        Self {
            direction: FlexDirection::Row,
            children,
            gap: 0.0,
        }
    }

    pub fn column(children: Vec<Box<dyn Widget<M>>>) -> Self {
        Self {
            direction: FlexDirection::Column,
            children,
            gap: 0.0,
        }
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

impl<M: 'static> Widget<M> for Flex<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let mut solver = FlexLayout::new(self.direction);
        solver.gap = self.gap;
        let (result, _) = solver.layout(constraints, &self.children, text_system, theme);
        result
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
    ) {
        let mut solver = FlexLayout::new(self.direction);
        solver.gap = self.gap;
        let (_result, child_rects) = solver.layout(
            Constraints::tight(rect.width, rect.height),
            &self.children,
            text_system,
            theme,
        );

        for (i, child) in self.children.iter().enumerate() {
            let mut child_rect = child_rects[i];
            child_rect.x += rect.x;
            child_rect.y += rect.y;
            child.draw(child_rect, draw_list, text_system, theme, focused_id);
        }
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        let mut solver = FlexLayout::new(self.direction);
        solver.gap = self.gap;
        let (_result, child_rects) = solver.layout(
            Constraints::tight(rect.width, rect.height),
            &self.children,
            text_system,
            theme,
        );

        for (i, child) in self.children.iter_mut().enumerate() {
            let mut child_rect = child_rects[i];
            child_rect.x += rect.x;
            child_rect.y += rect.y;

            messages.extend(child.on_event(event, child_rect, text_system, theme, focused_id));
        }
        messages
    }
}
