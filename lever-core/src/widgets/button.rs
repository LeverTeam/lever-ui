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
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    pub fn new(color: Color, hover_color: Color) -> Self {
        Self {
            color,
            hover_color,
            radius: 4.0,
            is_hovered: false,
            on_click: None,
        }
    }

    pub fn with_click<F>(mut self, f: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.on_click = Some(Box::new(f));
        self
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

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
    ) -> bool {
        match event {
            crate::event::FrameworkEvent::PointerMove { position } => {
                self.is_hovered = rect.contains(*position);
                false
            }
            crate::event::FrameworkEvent::PointerDown { position, button } => {
                if *button == crate::event::PointerButton::Primary {
                    if rect.contains(*position) {
                        if let Some(on_click) = &mut self.on_click {
                            on_click();
                        }
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}
