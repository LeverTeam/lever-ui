use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Rect, Size};
use crate::widget::Widget;

pub struct Center<M> {
    pub child: Box<dyn Widget<M>>,
    pub flex: u32,
}

impl<M> Center<M> {
    pub fn new(child: Box<dyn Widget<M>>) -> Self {
        Self { child, flex: 0 }
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }
}

impl<M: 'static> Widget<M> for Center<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let _child_res = self.child.layout(
            Constraints::loose(constraints.max_width, constraints.max_height),
            &[],
            text_system,
            theme,
        );

        LayoutResult {
            size: Size {
                width: constraints.max_width,
                height: constraints.max_height,
            },
        }
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
        let child_res = self.child.layout(
            Constraints::loose(rect.width, rect.height),
            &[],
            text_system,
            theme,
        );

        let child_rect = Rect {
            x: rect.x + (rect.width - child_res.size.width) / 2.0,
            y: rect.y + (rect.height - child_res.size.height) / 2.0,
            width: child_res.size.width,
            height: child_res.size.height,
        };

        self.child.draw(
            child_rect,
            draw_list,
            text_system,
            theme,
            focused_id,
            pointer_pos,
        );
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let child_res = self.child.layout(
            Constraints::loose(rect.width, rect.height),
            &[],
            text_system,
            theme,
        );

        let child_rect = Rect {
            x: rect.x + (rect.width - child_res.size.width) / 2.0,
            y: rect.y + (rect.height - child_res.size.height) / 2.0,
            width: child_res.size.width,
            height: child_res.size.height,
        };

        self.child
            .on_event(event, child_rect, text_system, theme, focused_id)
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
