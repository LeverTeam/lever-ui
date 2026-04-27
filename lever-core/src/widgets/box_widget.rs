use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{BoxShadow, Color, Gradient, Rect, SideOffsets, Size};
use crate::widget::Widget;

pub struct BoxWidget<M> {
    pub id: Option<String>,
    pub color: Color,
    pub gradient: Option<Gradient>,
    pub shadow: Option<BoxShadow>,
    pub radius: f32,
    pub padding: SideOffsets,
    pub alignment: crate::layout::Alignment,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub border_color: Option<Color>,
    pub border_thickness: f32,
    pub child: Option<Box<dyn Widget<M>>>,
    pub on_click: Option<Box<dyn Fn() -> M>>,
    pub flex: u32,
}

impl<M> BoxWidget<M> {
    pub fn new(color: Color) -> Self {
        Self {
            id: None,
            color,
            gradient: None,
            shadow: None,
            radius: 0.0,
            padding: SideOffsets::default(),
            alignment: crate::layout::Alignment::TopLeft,
            width: None,
            height: None,
            border_color: None,
            border_thickness: 0.0,
            child: None,
            on_click: None,
            flex: 0,
        }
    }

    pub fn transparent() -> Self {
        Self::new(Color::TRANSPARENT)
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_border(mut self, color: Color, thickness: f32) -> Self {
        self.border_color = Some(color);
        self.border_thickness = thickness;
        self
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

    pub fn with_alignment(mut self, alignment: crate::layout::Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn on_click<F>(mut self, on_click: F) -> Self
    where
        F: Fn() -> M + 'static,
    {
        self.on_click = Some(Box::new(on_click));
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
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
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

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
                self.width.unwrap_or(constraints.max_width)
                    - self.padding.left
                    - self.padding.right,
                self.height.unwrap_or(constraints.max_height)
                    - self.padding.top
                    - self.padding.bottom,
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

        if let Some(border_color) = self.border_color {
            draw_list.stroke_rect(rect, border_color, self.radius, self.border_thickness);
        }

        if let Some(child) = &self.child {
            let content_size = Size {
                width: rect.width - self.padding.left - self.padding.right,
                height: rect.height - self.padding.top - self.padding.bottom,
            };

            let child_res = child.layout(
                Constraints::loose(content_size.width, content_size.height),
                &[],
                text_system,
                theme,
            );

            let (ox, oy) = self.alignment.align(child_res.size, content_size);

            let child_rect = Rect {
                x: rect.x + self.padding.left + ox,
                y: rect.y + self.padding.top + oy,
                width: child_res.size.width,
                height: child_res.size.height,
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
        consumed: &mut bool,
    ) -> Vec<M> {
        let mut messages = Vec::new();

        if let Some(child) = &mut self.child {
            let content_size = Size {
                width: rect.width - self.padding.left - self.padding.right,
                height: rect.height - self.padding.top - self.padding.bottom,
            };

            let child_res = child.layout(
                Constraints::loose(content_size.width, content_size.height),
                &[],
                text_system,
                theme,
            );

            let (ox, oy) = self.alignment.align(child_res.size, content_size);

            let child_rect = Rect {
                x: rect.x + self.padding.left + ox,
                y: rect.y + self.padding.top + oy,
                width: child_res.size.width,
                height: child_res.size.height,
            };

            messages.extend(child.on_event(
                event,
                child_rect,
                text_system,
                theme,
                focused_id,
                consumed,
            ));

            if *consumed {
                return messages;
            }
        }

        if let Some(on_click) = &self.on_click {
            match event {
                crate::event::FrameworkEvent::PointerDown { position, .. } => {
                    if rect.contains(*position) {
                        *consumed = true;
                    }
                }
                crate::event::FrameworkEvent::PointerUp { position, .. } => {
                    if rect.contains(*position) {
                        *consumed = true;
                        messages.push(on_click());
                    }
                }
                _ => {}
            }
        }

        messages
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
