use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{BoxShadow, Color, Gradient, Rect, SideOffsets, Size};
use crate::widget::Widget;

pub struct BoxWidget<M> {
    pub color: Color,
    pub gradient: Option<Gradient>,
    pub shadow: Option<BoxShadow>,
    pub radius: f32,
    pub padding: SideOffsets,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub child: Option<Box<dyn Widget<M>>>,
    pub flex: u32,
}

impl<M> BoxWidget<M> {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            gradient: None,
            shadow: None,
            radius: 0.0,
            padding: SideOffsets::default(),
            width: None,
            height: None,
            child: None,
            flex: 0,
        }
    }

    pub fn with_gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    pub fn with_shadow(mut self, shadow: BoxShadow) -> Self {
        self.shadow = Some(shadow);
        self
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

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
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
        pointer_pos: Option<crate::types::Point>,
    ) {
        if let Some(gradient) = self.gradient {
            if let Some(shadow) = self.shadow {
                draw_list.shadowed_rect(rect, Color::rgba(0.0, 0.0, 0.0, 0.0), self.radius, shadow);
            }
            draw_list.gradient_rect(rect, gradient, self.radius);
        } else {
            if let Some(shadow) = self.shadow {
                draw_list.shadowed_rect(rect, self.color, self.radius, shadow);
            } else {
                draw_list.rounded_rect(rect, self.color, self.radius);
            }
        }

        if let Some(child) = &self.child {
            let child_rect = Rect {
                x: rect.x + self.padding.left,
                y: rect.y + self.padding.top,
                width: rect.width - self.padding.left - self.padding.right,
                height: rect.height - self.padding.top - self.padding.bottom,
            };
            child.draw(
                child_rect,
                draw_list,
                text_system,
                theme,
                focused_id,
                pointer_pos,
            );
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

    fn flex(&self) -> u32 {
        self.flex
    }
}
