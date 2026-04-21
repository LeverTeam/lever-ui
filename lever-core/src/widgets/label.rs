use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, Size};
use crate::widget::Widget;
use std::marker::PhantomData;

pub struct Label<M> {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    _marker: PhantomData<M>,
}

impl<M> Label<M> {
    pub fn new(text: impl Into<String>, font_size: f32, color: Color) -> Self {
        Self {
            text: text.into(),
            font_size,
            color,
            _marker: PhantomData,
        }
    }
}

impl<M: 'static> Widget<M> for Label<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let layout = text_system.shape(&self.text, self.font_size, self.color);
        LayoutResult {
            size: constraints.clamp_size(Size {
                width: layout.width,
                height: layout.height,
            }),
        }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
    ) {
        let layout = text_system.shape(&self.text, self.font_size, self.color);
        draw_list.text(
            crate::types::Point {
                x: rect.x,
                y: rect.y,
            },
            layout.glyphs,
        );
    }

    fn on_event(
        &mut self,
        _event: &crate::event::FrameworkEvent,
        _rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
    ) -> Vec<M> {
        Vec::new()
    }
}
