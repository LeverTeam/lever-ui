use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::Rect;
use crate::widget::Widget;

pub struct AnimatedOpacity<M> {
    pub opacity: f32,
    pub child: Box<dyn Widget<M>>,
}

impl<M> AnimatedOpacity<M> {
    pub fn new(opacity: f32, child: Box<dyn Widget<M>>) -> Self {
        Self { opacity, child }
    }
}

impl<M: 'static> Widget<M> for AnimatedOpacity<M> {
    fn id(&self) -> Option<&str> {
        self.child.id()
    }

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
    ) {
        if self.opacity <= 0.0 {
            return;
        }

        if self.opacity < 1.0 {
            draw_list.push_opacity(self.opacity);
        }

        self.child
            .draw(rect, draw_list, text_system, theme, focused_id);

        if self.opacity < 1.0 {
            draw_list.pop_opacity();
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        self.child
            .on_event(event, rect, text_system, theme, focused_id)
    }
}

pub struct AnimatedTranslation<M> {
    pub offset: crate::types::Point,
    pub child: Box<dyn Widget<M>>,
}

impl<M> AnimatedTranslation<M> {
    pub fn new(offset: crate::types::Point, child: Box<dyn Widget<M>>) -> Self {
        Self { offset, child }
    }
}

impl<M: 'static> Widget<M> for AnimatedTranslation<M> {
    fn id(&self) -> Option<&str> {
        self.child.id()
    }

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
    ) {
        draw_list.push_translation(self.offset);
        self.child
            .draw(rect, draw_list, text_system, theme, focused_id);
        draw_list.pop_translation();
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        self.child
            .on_event(event, rect, text_system, theme, focused_id)
    }
}
