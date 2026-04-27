use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, ImageFit, Rect, Size, TextureId};
use crate::widget::Widget;

pub struct ImageWidget<M> {
    pub texture: TextureId,
    pub size: Size,
    pub tint: Color,
    pub fit: ImageFit,
    pub flex: u32,
    _phantom: std::marker::PhantomData<M>,
}

impl<M> ImageWidget<M> {
    pub fn new(texture: TextureId) -> Self {
        Self {
            texture,
            size: Size::default(),
            tint: Color::rgb(1.0, 1.0, 1.0),
            fit: ImageFit::default(),
            flex: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = Size { width, height };
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.size.width = width;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.size.height = height;
        self
    }

    pub fn with_fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
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
        let size = if self.size.width > 0.0 && self.size.height > 0.0 {
            self.size
        } else {
            Size::new(100.0, 100.0)
        };

        LayoutResult {
            size: constraints.clamp_size(size),
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
        let uv = match self.fit {
            ImageFit::Fill => [0.0, 0.0, 1.0, 1.0],
            ImageFit::Contain | ImageFit::Cover => {
                let img_aspect = if self.size.width > 0.0 && self.size.height > 0.0 {
                    self.size.width / self.size.height
                } else {
                    1.0
                };
                let rect_aspect = rect.width / rect.height;

                if self.fit == ImageFit::Contain {
                    // Adjust rect to maintain aspect ratio within the given rect
                    let (target_w, target_h) = if rect_aspect > img_aspect {
                        (rect.height * img_aspect, rect.height)
                    } else {
                        (rect.width, rect.width / img_aspect)
                    };
                    let target_rect = Rect {
                        x: rect.x + (rect.width - target_w) / 2.0,
                        y: rect.y + (rect.height - target_h) / 2.0,
                        width: target_w,
                        height: target_h,
                    };
                    draw_list.textured_rect(
                        target_rect,
                        self.texture,
                        self.tint,
                        [0.0, 0.0, 1.0, 1.0],
                    );
                    return;
                } else {
                    // Cover: scale UVs to crop
                    if rect_aspect > img_aspect {
                        // Rect is wider than image: crop top/bottom
                        let uv_h = img_aspect / rect_aspect;
                        let uv_y = (1.0 - uv_h) / 2.0;
                        [0.0, uv_y, 1.0, uv_h]
                    } else {
                        // Rect is taller than image: crop left/right
                        let uv_w = rect_aspect / img_aspect;
                        let uv_x = (1.0 - uv_w) / 2.0;
                        [uv_x, 0.0, uv_w, 1.0]
                    }
                }
            }
        };

        draw_list.textured_rect(rect, self.texture, self.tint, uv);
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
