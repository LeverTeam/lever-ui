use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, SideOffsets};
use crate::widget::Widget;

pub struct BoxWidget {
    pub color: Color,
    pub radius: f32,
    pub padding: SideOffsets,
    pub child: Option<Box<dyn Widget>>,
}

impl BoxWidget {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            radius: 0.0,
            padding: SideOffsets::default(),
            child: None,
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_padding(mut self, padding: SideOffsets) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_child(mut self, child: Box<dyn Widget>) -> Self {
        self.child = Some(child);
        self
    }
}

impl Widget for BoxWidget {
    fn build(&self) -> Vec<Box<dyn Widget>> {
        if let Some(_child) = &self.child {}
        Vec::new()
    }

    fn layout(&self, constraints: Constraints, _children: &[LayoutNode]) -> LayoutResult {
        let mut content_constraints = constraints;
        content_constraints.max_width -= self.padding.left + self.padding.right;
        content_constraints.max_height -= self.padding.top + self.padding.bottom;

        let child_size = if let Some(child) = &self.child {
            child.layout(content_constraints, &[]).size
        } else {
            crate::types::Size {
                width: 0.0,
                height: 0.0,
            }
        };

        let final_size = constraints.clamp_size(crate::types::Size {
            width: child_size.width + self.padding.left + self.padding.right,
            height: child_size.height + self.padding.top + self.padding.bottom,
        });

        LayoutResult { size: final_size }
    }

    fn draw(&self, rect: Rect, draw_list: &mut DrawList) {
        if self.radius > 0.0 {
            draw_list.rounded_rect(rect, self.color, self.radius);
        } else {
            draw_list.colored_rect(rect, self.color, 0.0);
        }

        if let Some(child) = &self.child {
            let child_rect = Rect {
                x: rect.x + self.padding.left,
                y: rect.y + self.padding.top,
                width: rect.width - (self.padding.left + self.padding.right),
                height: rect.height - (self.padding.top + self.padding.bottom),
            };
            child.draw(child_rect, draw_list);
        }
    }
}
