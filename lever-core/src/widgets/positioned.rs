use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{PositionedOffset, Rect};
use crate::widget::Widget;
use std::marker::PhantomData;

pub struct Positioned<M> {
    pub child: Box<dyn Widget<M>>,
    pub offset: PositionedOffset,
    _marker: PhantomData<M>,
}

impl<M> Positioned<M> {
    pub fn new(child: Box<dyn Widget<M>>) -> Self {
        Self {
            child,
            offset: PositionedOffset::default(),
            _marker: PhantomData,
        }
    }

    pub fn top(mut self, value: f32) -> Self {
        self.offset.top = Some(value);
        self
    }

    pub fn bottom(mut self, value: f32) -> Self {
        self.offset.bottom = Some(value);
        self
    }

    pub fn left(mut self, value: f32) -> Self {
        self.offset.left = Some(value);
        self
    }

    pub fn right(mut self, value: f32) -> Self {
        self.offset.right = Some(value);
        self
    }
}

impl<M: 'static> Widget<M> for Positioned<M> {
    fn layout(
        &self,
        constraints: Constraints,
        children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        self.child.layout(constraints, children, text_system, theme)
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        self.child
            .draw(rect, draw_list, text_system, theme, focused_id, pointer_pos);
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        self.child
            .on_event(event, rect, text_system, theme, focused_id, consumed)
    }

    fn positioned(&self) -> Option<PositionedOffset> {
        Some(self.offset)
    }
}
