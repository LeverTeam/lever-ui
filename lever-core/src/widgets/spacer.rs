use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Rect, Size};
use crate::widget::Widget;
use std::marker::PhantomData;

pub struct Spacer<M> {
    pub width: f32,
    pub height: f32,
    pub flex: u32,
    _marker: PhantomData<M>,
}

impl<M> Spacer<M> {
    pub fn new() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            flex: 0,
            _marker: PhantomData,
        }
    }

    pub fn width(width: f32) -> Self {
        Self {
            width,
            height: 0.0,
            flex: 0,
            _marker: PhantomData,
        }
    }

    pub fn height(height: f32) -> Self {
        Self {
            width: 0.0,
            height,
            flex: 0,
            _marker: PhantomData,
        }
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
        _rect: Rect,
        _draw_list: &mut DrawList,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        _pointer_pos: Option<crate::types::Point>,
    ) {
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


