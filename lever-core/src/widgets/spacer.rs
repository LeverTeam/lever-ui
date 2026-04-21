use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::Rect;
use crate::widget::Widget;

pub struct Spacer {
    pub flex: u32,
}

impl Spacer {
    pub fn new() -> Self {
        Self { flex: 1 }
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }
}

impl Widget for Spacer {
    fn flex(&self) -> u32 {
        self.flex
    }

    fn build(&self) -> Vec<Box<dyn Widget>> {
        Vec::new()
    }

    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        LayoutResult {
            size: crate::types::Size {
                width: constraints.min_width,
                height: constraints.min_height,
            },
        }
    }

    fn draw(
        &self,
        _rect: Rect,
        _draw_list: &mut DrawList,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) {
    }
}
