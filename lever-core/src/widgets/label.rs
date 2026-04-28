use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, Size, TextAlign};
use crate::widget::Widget;
use std::marker::PhantomData;

pub struct Label<M> {
    pub text: String,
    pub font_size: Option<f32>,
    pub color: Option<Color>,
    pub align: TextAlign,
    pub flex: u32,
    _marker: PhantomData<M>,
}

impl<M> Label<M> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: None,
            color: None,
            align: TextAlign::default(),
            flex: 0,
            _marker: PhantomData,
        }
    }

    pub fn with_size(mut self, font_size: f32) -> Self {
        self.font_size = Some(font_size);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
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
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let font_size = self.font_size.unwrap_or(theme.font_size_md);
        let color = self.color.unwrap_or(theme.text);

        let layout = text_system.shape(
            &self.text,
            font_size,
            color,
            constraints.max_width_opt(),
            TextAlign::Left, // Alignment is handled during drawing
        );
        LayoutResult {
            size: constraints.clamp_size(Size {
                width: layout.width.ceil() + 4.0,
                height: layout.height.ceil(),
            }),
        }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        _pointer_pos: Option<crate::types::Point>,
    ) {
        let font_size = self.font_size.unwrap_or(theme.font_size_md);
        let color = self.color.unwrap_or(theme.text);

        let layout = text_system.shape(&self.text, font_size, color, Some(rect.width), self.align);
        draw_list.text(
            crate::types::Point {
                x: rect.x.round(),
                y: (rect.y + (rect.height - layout.height) / 2.0).round(),
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
        _consumed: &mut bool,
    ) -> Vec<M> {
        Vec::new()
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
