use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{BoxShadow, Color, Point, Rect, Size};
use crate::widget::Widget;

pub struct Button<M> {
    pub label: String,
    pub color: Option<Color>,
    pub on_click: Option<Box<dyn Fn() -> M>>,
}

impl<M> Button<M> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: None,
            on_click: None,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() -> M + 'static,
    {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Button<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let size = constraints.clamp_size(Size {
            width: 120.0,
            height: 40.0,
        });
        LayoutResult { size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        let is_hovered = pointer_pos.map_or(false, |pos| rect.contains(pos));
        let base_color = self.color.unwrap_or(theme.primary);

        let button_color = if is_hovered {
            Color {
                r: (base_color.r * 1.1).min(1.0),
                g: (base_color.g * 1.1).min(1.0),
                b: (base_color.b * 1.1).min(1.0),
                a: base_color.a,
            }
        } else {
            base_color
        };

        // Subtle shadow
        draw_list.shadowed_rect(
            rect,
            button_color,
            theme.radius_md,
            BoxShadow {
                offset: Point { x: 0.0, y: 2.0 },
                blur: 6.0,
                color: theme.shadow_color,
            },
        );

        // Label
        let layout = text_system.shape(&self.label, 14.0, theme.on_primary);
        let x_offset = (rect.width - layout.width) / 2.0;
        let y_offset = (rect.height - layout.height) / 2.0;

        draw_list.text(
            Point {
                x: rect.x + x_offset,
                y: rect.y + y_offset,
            },
            layout.glyphs,
        );
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            crate::event::FrameworkEvent::PointerDown { position, .. } => {
                if rect.contains(*position) {
                    if let Some(on_click) = &self.on_click {
                        messages.push(on_click());
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
