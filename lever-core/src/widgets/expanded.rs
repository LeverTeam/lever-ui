use crate::FrameworkEvent;
use crate::draw::DrawList;
use crate::layout::{Alignment, Constraints, LayoutNode, LayoutResult};
use crate::types::Rect;
use crate::widget::Widget;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexFit {
    #[default]
    Tight,
    Loose,
}

pub struct Expanded<M> {
    pub flex: u32,
    pub fit: FlexFit,
    pub alignment: Alignment,
    pub child: Box<dyn Widget<M>>,
}

impl<M> Expanded<M> {
    pub fn new(child: Box<dyn Widget<M>>) -> Self {
        Self {
            flex: 1,
            fit: FlexFit::Tight,
            alignment: Alignment::Center,
            child,
        }
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }

    pub fn with_fit(mut self, fit: FlexFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
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
        // Expanded always takes all the space it's given by Flex
        let child_constraints = match self.fit {
            FlexFit::Tight => constraints, // Pass the tight constraints (usually fixed size from Flex)
            FlexFit::Loose => Constraints::loose(constraints.max_width, constraints.max_height),
        };

        let _res = self
            .child
            .layout(child_constraints, &[], text_system, theme);

        LayoutResult {
            size: constraints.max_size(),
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
        // Re-layout child to get its size for alignment
        let child_constraints = match self.fit {
            FlexFit::Tight => Constraints::tight(rect.width, rect.height),
            FlexFit::Loose => Constraints::loose(rect.width, rect.height),
        };
        let res = self
            .child
            .layout(child_constraints, &[], text_system, theme);

        let (dx, dy) = self.alignment.align(res.size, rect.size());
        let child_rect = Rect {
            x: rect.x + dx,
            y: rect.y + dy,
            width: res.size.width,
            height: res.size.height,
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

        let child_rect = Rect {
            x: rect.x + (rect.width - child_res.size.width) / 2.0,
            y: rect.y + (rect.height - child_res.size.height) / 2.0,
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
