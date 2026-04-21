use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::Rect;
use crate::widget::Widget;

pub struct Expanded<M> {
    pub flex: u32,
    pub child: Box<dyn Widget<M>>,
}

impl<M> Expanded<M> {
    pub fn new(child: Box<dyn Widget<M>>) -> Self {
        Self { flex: 1, child }
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }
}

impl<M: 'static> Widget<M> for Expanded<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        self.child.layout(constraints, &[], text_system, theme)
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
    ) {
        self.child
            .draw(rect, draw_list, text_system, theme, focused_id);
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        self.child
            .on_event(event, rect, text_system, theme, focused_id)
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
