use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Rect, Size};
use crate::widget::Widget;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpacerOrientation {
    Auto,
    Horizontal,
    Vertical,
}

pub struct Spacer<M> {
    pub width: f32,
    pub height: f32,
    pub flex: u32,
    pub divider_color: Option<crate::types::Color>,
    pub thickness: f32,
    pub orientation: SpacerOrientation,
    _marker: PhantomData<M>,
}

impl<M> Spacer<M> {
    pub fn new() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            flex: 0,
            divider_color: None,
            thickness: 1.0,
            orientation: SpacerOrientation::Auto,
            _marker: PhantomData,
        }
    }

    pub fn flex() -> Self {
        Self::new().with_flex(1)
    }

    pub fn horizontal(width: f32) -> Self {
        Self::new().with_size(width, 0.0)
    }

    pub fn vertical(height: f32) -> Self {
        Self::new().with_size(0.0, height)
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }

    pub fn with_divider(mut self, color: crate::types::Color) -> Self {
        self.divider_color = Some(color);
        self
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_orientation(mut self, orientation: SpacerOrientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl<M: 'static> Widget<M> for Spacer<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        LayoutResult {
            size: constraints.clamp_size(Size {
                width: self.width,
                height: self.height,
            }),
        }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        _pointer_pos: Option<crate::types::Point>,
    ) {
        if let Some(color) = self.divider_color {
            let is_horizontal = match self.orientation {
                SpacerOrientation::Horizontal => true,
                SpacerOrientation::Vertical => false,
                SpacerOrientation::Auto => rect.width > rect.height,
            };

            if is_horizontal {
                // Horizontal divider (centered vertically)
                let draw_w = if self.width > 0.0 {
                    self.width.min(rect.width)
                } else {
                    rect.width
                };
                let draw_x = rect.x + (rect.width - draw_w) / 2.0;

                draw_list.rounded_rect(
                    Rect {
                        x: draw_x,
                        y: rect.y + (rect.height - self.thickness) / 2.0,
                        width: draw_w,
                        height: self.thickness,
                    },
                    color,
                    0.0,
                );
            } else {
                // Vertical divider (centered horizontally and vertically)
                let draw_h = if self.height > 0.0 {
                    self.height.min(rect.height)
                } else {
                    rect.height
                };
                let draw_y = rect.y + (rect.height - draw_h) / 2.0;

                draw_list.rounded_rect(
                    Rect {
                        x: rect.x + (rect.width - self.thickness) / 2.0,
                        y: draw_y,
                        width: self.thickness,
                        height: draw_h,
                    },
                    color,
                    0.0,
                );
            }
        }
    }

    fn on_event(
        &mut self,
        _event: &crate::event::FrameworkEvent,
        _rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
        _consumed: &mut bool,
    ) -> Vec<M> {
        Vec::new()
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
