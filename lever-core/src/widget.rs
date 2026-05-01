use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::Rect;

pub trait Widget<M> {
    fn build(&self) -> Vec<Box<dyn Widget<M>>> {
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
        focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    );

    fn on_event(
        &mut self,
        _event: &crate::event::FrameworkEvent,
        _rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
        _consumed: &mut bool,
    ) -> Vec<M> {
        Vec::new()
    }

    fn hit_test(&self, pos: crate::types::Point, rect: Rect) -> bool {
        rect.contains(pos)
    }

    fn flex(&self) -> u32 {
        0
    }

    fn id(&self) -> Option<&str> {
        None
    }

    fn positioned(&self) -> Option<crate::types::PositionedOffset> {
        None
    }

    fn tick(&mut self, _dt: f32) {}
}
