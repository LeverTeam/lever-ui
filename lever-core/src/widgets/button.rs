use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect};
use crate::widget::Widget;

pub struct Button {
    pub color: Option<Color>,
    pub hover_color: Option<Color>,
    pub radius: Option<f32>,
    pub is_hovered: bool,
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            color: None,
            hover_color: None,
            radius: None,
            is_hovered: false,
            on_click: None,
        }
    }

    pub fn with_colors(mut self, color: Color, hover_color: Color) -> Self {
        self.color = Some(color);
        self.hover_color = Some(hover_color);
        self
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
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
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
        theme: &crate::theme::Theme,
    ) {
        let color = if self.is_hovered {
            self.hover_color.unwrap_or(theme.primary_hover)
        } else {
            self.color.unwrap_or(theme.primary)
        };
        draw_list.rounded_rect(rect, color, self.radius.unwrap_or(theme.radius_md));
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
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
