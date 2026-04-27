use crate::widget::Widget;
use crate::widgets::{Expanded, FlexFit};

pub struct Flexible<M> {
    pub expanded: Expanded<M>,
}

impl<M> Flexible<M> {
    pub fn new(child: Box<dyn Widget<M>>) -> Self {
        Self {
            expanded: Expanded::new(child).with_fit(FlexFit::Loose),
        }
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.expanded.flex = flex;
        self
    }
}

impl<M: 'static> Widget<M> for Flexible<M> {
    fn layout(
        &self,
        constraints: crate::layout::Constraints,
        children: &[crate::layout::LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> crate::layout::LayoutResult {
        self.expanded
            .layout(constraints, children, text_system, theme)
    }

    fn draw(
        &self,
        rect: crate::types::Rect,
        draw_list: &mut crate::draw::DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        self.expanded
            .draw(rect, draw_list, text_system, theme, focused_id, pointer_pos)
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: crate::types::Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        self.expanded
            .on_event(event, rect, text_system, theme, focused_id, consumed)
    }

    fn flex(&self) -> u32 {
        self.expanded.flex()
    }

    fn id(&self) -> Option<&str> {
        self.expanded.id()
    }
}
