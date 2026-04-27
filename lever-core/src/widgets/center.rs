use crate::FrameworkEvent;
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
        // Center always wants to fill the available space if possible
        let size = Size {
            width: if constraints.max_width.is_finite() {
                constraints.max_width
            } else {
                0.0
            },
            height: if constraints.max_height.is_finite() {
                constraints.max_height
            } else {
                0.0
            },
        };

        self.child.layout(
            Constraints::loose(size.width, size.height),
            &[],
            text_system,
            theme,
        );

        LayoutResult { size }
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
        event: &FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let child_res = self.child.layout(
            Constraints::loose(rect.width, rect.height),
            &[],
            text_system,
            theme,
        );

        let ox = (rect.width - child_res.size.width) / 2.0;
        let oy = (rect.height - child_res.size.height) / 2.0;
        let child_rect = Rect {
            x: rect.x + ox,
            y: rect.y + oy,
            width: child_res.size.width,
            height: child_res.size.height,
        };

        self.child
            .on_event(event, child_rect, text_system, theme, focused_id, consumed)
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
