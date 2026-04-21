use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect};
use crate::widget::Widget;
use crate::widgets::box_widget::BoxWidget;

pub struct Button {
    pub color: Color,
    pub hover_color: Color,
    pub radius: f32,
    pub is_hovered: bool,
}

impl Button {
    pub fn new(color: Color, hover_color: Color) -> Self {
        Self {
            color,
            hover_color,
            radius: 4.0,
            is_hovered: false,
        }
    }
}

impl Widget for Button {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
    ) -> LayoutResult {
        let size = constraints.clamp_size(crate::types::Size {
            width: 100.0,
            height: 40.0,
        });
        LayoutResult { size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        _text_system: &mut crate::text::TextSystem,
    ) {
        let color = if self.is_hovered {
            self.hover_color
        } else {
            self.color
        };
        draw_list.rounded_rect(rect, color, self.radius);
    }
}
