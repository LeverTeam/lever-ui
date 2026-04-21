use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Point, Rect, Size};
use crate::widget::Widget;

pub struct Label {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
}

impl Label {
    pub fn new(text: impl Into<String>, font_size: f32, color: Color) -> Self {
        Self {
            text: text.into(),
            font_size,
            color,
        }
    }
}

impl Widget for Label {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
    ) -> LayoutResult {
        let layout = text_system.shape(&self.text, self.font_size, self.color);
        let size = constraints.clamp_size(Size {
            width: layout.width,
            height: layout.height,
        });
        LayoutResult { size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
    ) {
        let layout = text_system.shape(&self.text, self.font_size, self.color);

        draw_list.text(
            Point {
                x: rect.x,
                y: rect.y,
            },
            layout.glyphs,
        );
    }
}
