use crate::animated::{animated_color, animated_spring};
use crate::animation::Spring;
use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{BoxShadow, Color, Point, Rect, Size};
use crate::widget::Widget;

pub struct Button<M> {
    pub id: Option<String>,
    pub label: String,
    pub color: Option<Color>,
    pub flex: u32,
    pub on_click: Option<Box<dyn Fn() -> M>>,
}

impl<M> Button<M> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: None,
            label: label.into(),
            color: None,
            flex: 0,
            on_click: None,
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
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

        // Animate color and scale if ID is provided
        let (button_color, scale) = if let Some(id) = &self.id {
            let target_color = if is_hovered {
                Color {
                    r: (base_color.r * 1.1).min(1.0),
                    g: (base_color.g * 1.1).min(1.0),
                    b: (base_color.b * 1.1).min(1.0),
                    a: base_color.a,
                }
            } else {
                base_color
            };

            let animated_c = animated_color(&format!("{}_color", id), target_color, 0.15);
            let target_scale = if is_hovered { 1.05 } else { 1.0 };
            let animated_s =
                animated_spring(&format!("{}_scale", id), target_scale, Spring::SNAPPY);

            (animated_c, animated_s)
        } else {
            let c = if is_hovered {
                Color {
                    r: (base_color.r * 1.1).min(1.0),
                    g: (base_color.g * 1.1).min(1.0),
                    b: (base_color.b * 1.1).min(1.0),
                    a: base_color.a,
                }
            } else {
                base_color
            };
            (c, 1.0)
        };

        // Apply scale to the rect
        let scaled_rect = if scale != 1.0 {
            let new_w = rect.width * scale;
            let new_h = rect.height * scale;
            Rect {
                x: rect.x - (new_w - rect.width) / 2.0,
                y: rect.y - (new_h - rect.height) / 2.0,
                width: new_w,
                height: new_h,
            }
        } else {
            rect
        };

        // Subtle shadow
        draw_list.shadowed_rect(
            scaled_rect,
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
        let x = rect.x + (rect.width - layout.width) / 2.0;
        let y = rect.y + (rect.height - layout.height) / 2.0;

        draw_list.text(
            Point {
                x: x.round(),
                y: y.round(),
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

    fn flex(&self) -> u32 {
        self.flex
    }
}
