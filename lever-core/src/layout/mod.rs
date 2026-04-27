pub mod constraints;
pub mod flex;
pub mod grid;

use crate::types::{Rect, Size};
pub use constraints::*;
pub use flex::*;
pub use grid::*;

#[derive(Debug, Clone, Copy)]
pub struct Constraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Constraints {
    pub fn loose(max_width: f32, max_height: f32) -> Self {
        Self {
            min_width: 0.0,
            max_width: max_width.max(0.0),
            min_height: 0.0,
            max_height: max_height.max(0.0),
        }
    }

    pub fn max_size(&self) -> Size {
        Size {
            width: self.max_width,
            height: self.max_height,
        }
    }

    pub fn tight(width: f32, height: f32) -> Self {
        let w = width.max(0.0);
        let h = height.max(0.0);
        Self {
            min_width: w,
            max_width: w,
            min_height: h,
            max_height: h,
        }
    }

    pub fn clamp_size(&self, size: Size) -> Size {
        let min_w = self.min_width;
        let max_w = self.max_width.max(min_w);
        let min_h = self.min_height;
        let max_h = self.max_height.max(min_h);
        Size {
            width: size.width.clamp(min_w, max_w),
            height: size.height.clamp(min_h, max_h),
        }
    }

    pub fn max_width_opt(&self) -> Option<f32> {
        if self.max_width.is_finite() {
            Some(self.max_width)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutResult {
    pub size: Size,
}

pub struct LayoutNode {
    pub rect: Rect,
    pub children: Vec<LayoutNode>,
}

impl LayoutNode {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Alignment {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Alignment {
    pub fn align(&self, child_size: Size, parent_size: Size) -> (f32, f32) {
        let dx = parent_size.width - child_size.width;
        let dy = parent_size.height - child_size.height;

        match self {
            Alignment::TopLeft => (0.0, 0.0),
            Alignment::TopCenter => (dx / 2.0, 0.0),
            Alignment::TopRight => (dx, 0.0),
            Alignment::CenterLeft => (0.0, dy / 2.0),
            Alignment::Center => (dx / 2.0, dy / 2.0),
            Alignment::CenterRight => (dx, dy / 2.0),
            Alignment::BottomLeft => (0.0, dy),
            Alignment::BottomCenter => (dx / 2.0, dy),
            Alignment::BottomRight => (dx, dy),
        }
    }
}
