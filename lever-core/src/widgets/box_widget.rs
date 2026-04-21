use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, SideOffsets, Size};
use crate::widget::Widget;

pub struct BoxWidget<M> {
    pub color: Color,
    pub radius: f32,
    pub padding: SideOffsets,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub child: Option<Box<dyn Widget<M>>>,
}

impl<M> BoxWidget<M> {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            radius: 0.0,
            padding: SideOffsets::default(),
            width: None,
            height: None,
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

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_child(mut self, child: Box<dyn Widget<M>>) -> Self {
        self.child = Some(child);
        self
    }
}

impl<M: 'static> Widget<M> for BoxWidget<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let mut size = Size {
            width: self.width.unwrap_or(0.0),
            height: self.height.unwrap_or(0.0),
        };

        if let Some(child) = &self.child {
            let child_constraints = Constraints::loose(
                constraints.max_width - self.padding.left - self.padding.right,
                constraints.max_height - self.padding.top - self.padding.bottom,
            );
            let child_res = child.layout(child_constraints, &[], text_system, theme);
            size.width = size
                .width
                .max(child_res.size.width + self.padding.left + self.padding.right);
            size.height = size
                .height
                .max(child_res.size.height + self.padding.top + self.padding.bottom);
        }

        LayoutResult {
            size: constraints.clamp_size(size),
        }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
    ) {
        draw_list.rounded_rect(rect, self.color, self.radius);

        if let Some(child) = &self.child {
            let child_rect = Rect {
                x: rect.x + self.padding.left,
                y: rect.y + self.padding.top,
                width: rect.width - self.padding.left - self.padding.right,
                height: rect.height - self.padding.top - self.padding.bottom,
            };
            child.draw(child_rect, draw_list, text_system, theme, focused_id);
        }
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        if let Some(child) = &mut self.child {
            let child_rect = Rect {
                x: rect.x + self.padding.left,
                y: rect.y + self.padding.top,
                width: rect.width - self.padding.left - self.padding.right,
                height: rect.height - self.padding.top - self.padding.bottom,
            };
            child.on_event(event, child_rect, text_system, theme, focused_id)
        } else {
            Vec::new()
        }
    }
}
