pub mod flex;
pub mod grid;

use crate::types::{Rect, Size};
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
