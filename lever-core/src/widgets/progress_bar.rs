use crate::animated::animated_spring;
use crate::animation::Spring;
use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, Size};
use crate::widget::Widget;

pub struct ProgressBar<M> {
    pub id: String,
    pub progress: f32,
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub thickness: f32,
    pub radius: Option<f32>,
    pub indeterminate: bool,
    pub flex: u32,
    _phantom: std::marker::PhantomData<M>,
}

impl<M> ProgressBar<M> {
    pub fn new(id: impl Into<String>, progress: f32) -> Self {
        Self {
            id: id.into(),
            progress: progress.clamp(0.0, 1.0),
            color: None,
            background_color: None,
            thickness: 6.0,
            radius: None,
            indeterminate: false,
            flex: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn indeterminate(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            progress: 0.0,
            color: None,
            background_color: None,
            thickness: 6.0,
            radius: None,
            indeterminate: true,
            flex: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }
}

impl<M: 'static> Widget<M> for ProgressBar<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let width = constraints.max_width;
        let height = self.thickness.max(constraints.min_height);

        LayoutResult {
            size: constraints.clamp_size(Size::new(width, height)),
        }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        _text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        _pointer_pos: Option<crate::types::Point>,
    ) {
        let radius = self.radius.unwrap_or(self.thickness / 2.0);
        let bar_color = self.color.unwrap_or(theme.primary);
        let bg_color = self.background_color.unwrap_or(theme.surface_variant);

        draw_list.rounded_rect(rect, bg_color, radius);

        if self.indeterminate {
            let time = crate::state::get_time();
            let anim_offset = (time * 0.8).fract();

            let width = rect.width * 0.3;
            let x = rect.x + (rect.width + width) * anim_offset - width;

            let bar_rect = Rect {
                x: x.max(rect.x),
                y: rect.y,
                width: if x < rect.x {
                    width + (x - rect.x)
                } else if x + width > rect.x + rect.width {
                    (rect.x + rect.width) - x
                } else {
                    width
                }
                .max(0.0),
                height: rect.height,
            };

            if bar_rect.width > 0.0 {
                draw_list.rounded_rect(bar_rect, bar_color, radius);
            }
        } else {
            let animated_progress = animated_spring(
                &format!("{}_progress", self.id),
                self.progress,
                Spring::SMOOTH,
            );

            if animated_progress > 0.001 {
                let bar_rect = Rect {
                    x: rect.x,
                    y: rect.y,
                    width: rect.width * animated_progress,
                    height: rect.height,
                };

                draw_list.rounded_rect(bar_rect, bar_color, radius);
            }
        }
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}

pub struct CircularProgress<M> {
    pub id: String,
    pub progress: f32,
    pub color: Option<Color>,
    pub thickness: f32,
    pub size: f32,
    pub indeterminate: bool,
    pub flex: u32,
    _phantom: std::marker::PhantomData<M>,
}

impl<M> CircularProgress<M> {
    pub fn new(id: impl Into<String>, progress: f32) -> Self {
        Self {
            id: id.into(),
            progress: progress.clamp(0.0, 1.0),
            color: None,
            thickness: 4.0,
            size: 40.0,
            indeterminate: false,
            flex: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn indeterminate(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            progress: 0.0,
            color: None,
            thickness: 4.0,
            size: 40.0,
            indeterminate: true,
            flex: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }
}

impl<M: 'static> Widget<M> for CircularProgress<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        LayoutResult {
            size: constraints.clamp_size(Size::new(self.size, self.size)),
        }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        _text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        _pointer_pos: Option<crate::types::Point>,
    ) {
        let color = self.color.unwrap_or(theme.primary);

        if self.indeterminate {
            let time = crate::state::get_time();
            draw_list.push_scale(1.0, rect.center());

            let progress = (time * 0.8).fract();
            draw_list.arc(rect, color, self.thickness, progress);
        } else {
            let animated_progress = animated_spring(
                &format!("{}_progress", self.id),
                self.progress,
                Spring::SMOOTH,
            );

            draw_list.arc(rect, color, self.thickness, animated_progress);
        }
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
