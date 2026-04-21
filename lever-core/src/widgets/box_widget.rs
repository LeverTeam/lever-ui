use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, SideOffsets};
use crate::widget::Widget;

pub struct BoxWidget {
    pub color: Color,
    pub padding: SideOffsets,
    pub border_radius: f32,
    pub child: Option<Box<dyn Widget>>,
    pub gradient: Option<crate::types::Gradient>,
    pub shadow: Option<crate::types::BoxShadow>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl BoxWidget {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            padding: SideOffsets::default(),
            border_radius: 0.0,
            child: None,
            gradient: None,
            shadow: None,
            width: None,
            height: None,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
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

    pub fn with_gradient(mut self, gradient: crate::types::Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    pub fn with_shadow(mut self, shadow: crate::types::BoxShadow) -> Self {
        self.shadow = Some(shadow);
        self
    }
}

impl Widget for BoxWidget {
    fn build(&self) -> Vec<Box<dyn Widget>> {
        if let Some(_child) = &self.child {}
        Vec::new()
    }

    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let padding_w = self.padding.left + self.padding.right;
        let padding_h = self.padding.top + self.padding.bottom;

        let content_constraints = Constraints {
            min_width: (constraints.min_width - padding_w).max(0.0),
            max_width: (constraints.max_width - padding_w).max(0.0),
            min_height: (constraints.min_height - padding_h).max(0.0),
            max_height: (constraints.max_height - padding_h).max(0.0),
        };

        let child_size = if let Some(child) = &self.child {
            child
                .layout(content_constraints, &[], text_system, theme)
                .size
        } else {
            crate::types::Size {
                width: 0.0,
                height: 0.0,
            }
        };

        let mut final_size = constraints.clamp_size(crate::types::Size {
            width: child_size.width + padding_w,
            height: child_size.height + padding_h,
        });

        if let Some(w) = self.width {
            final_size.width = w;
        }
        if let Some(h) = self.height {
            final_size.height = h;
        }

        LayoutResult { size: final_size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) {
        if let Some(gradient) = self.gradient {
            if let Some(shadow) = self.shadow {
                draw_list.shadowed_rect(
                    rect,
                    Color::rgba(0.0, 0.0, 0.0, 0.0),
                    self.border_radius,
                    shadow,
                );
            }
            draw_list.gradient_rect(rect, gradient, self.border_radius);
        } else {
            draw_list
                .commands_mut()
                .push(crate::draw::DrawCommand::RoundedRect {
                    rect,
                    color: self.color,
                    radius: self.border_radius,
                    shadow: self.shadow,
                });
        }

        if let Some(child) = &self.child {
            let child_rect = Rect {
                x: rect.x + self.padding.left,
                y: rect.y + self.padding.top,
                width: rect.width - (self.padding.left + self.padding.right),
                height: rect.height - (self.padding.top + self.padding.bottom),
            };
            child.draw(child_rect, draw_list, text_system, theme);
        }
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> bool {
        if let Some(child) = &mut self.child {
            let child_rect = Rect {
                x: rect.x + self.padding.left,
                y: rect.y + self.padding.top,
                width: rect.width - (self.padding.left + self.padding.right),
                height: rect.height - (self.padding.top + self.padding.bottom),
            };
            return child.on_event(event, child_rect, text_system, theme);
        }
        false
    }
}
