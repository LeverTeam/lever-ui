use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, Size};
use crate::widget::Widget;
use std::marker::PhantomData;

pub struct Label<M> {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub flex: u32,
    _marker: PhantomData<M>,
}

impl<M> Label<M> {
    pub fn new(text: impl Into<String>, font_size: f32, color: Color) -> Self {
        Self {
            text: text.into(),
            font_size,
            color,
            flex: 0,
            _marker: PhantomData,
        }
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
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
        _pointer_pos: Option<crate::types::Point>,
    ) {
        let layout = text_system.shape(&self.text, self.font_size, self.color);
        draw_list.text(
            crate::types::Point {
                x: rect.x.round(),
                y: rect.y.round(),
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

    fn flex(&self) -> u32 {
        self.flex
    }
}
