use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, Size, TextureId};
use crate::widget::Widget;

pub struct ImageWidget<M> {
    pub texture: TextureId,
    pub size: Size,
    pub tint: Color,
    pub flex: u32,
    _phantom: std::marker::PhantomData<M>,
}

impl<M> ImageWidget<M> {
    pub fn new(texture: TextureId, size: Size) -> Self {
        Self {
            texture,
            size,
            tint: Color::rgb(1.0, 1.0, 1.0),
            flex: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_tint(mut self, color: Color) -> Self {
        self.tint = color;
        self
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }
}

impl<M: 'static> Widget<M> for ImageWidget<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        LayoutResult {
            size: constraints.clamp_size(self.size),
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
        draw_list.textured_rect(rect, self.texture, self.tint, [0.0, 0.0, 1.0, 1.0]);
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
