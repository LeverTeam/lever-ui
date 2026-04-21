use crate::tessellation::Tessellator;
use lever_core::types::{Color, Rect};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ColoredVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

pub struct RectBatch {
    vertices: Vec<ColoredVertex>,
    indices: Vec<u32>,
    tess: Tessellator,
}

impl RectBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            tess: Tessellator::new(),
        }
    }

    pub fn push_rect(&mut self, rect: Rect, color: Color) {
        let x1 = rect.x;
        let y1 = rect.y;
        let x2 = rect.x + rect.width;
        let y2 = rect.y + rect.height;

        let c = color.to_array();
        let start_index = self.vertices.len() as u32;

        self.vertices.push(ColoredVertex {
            position: [x1, y1],
            color: c,
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c,
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y2],
            color: c,
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c,
        });

        self.indices.push(start_index);
        self.indices.push(start_index + 1);
        self.indices.push(start_index + 2);
        self.indices.push(start_index);
        self.indices.push(start_index + 2);
        self.indices.push(start_index + 3);
    }

    pub fn push_rounded_rect(&mut self, rect: Rect, radius: f32, color: Color) {
        if radius <= 0.0 {
            self.push_rect(rect, color);
        } else {
            self.tess.tessellate_rounded_rect(
                rect,
                radius,
                color,
                &mut self.vertices,
                &mut self.indices,
            );
        }
    }

    pub fn vertices(&self) -> &[ColoredVertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}
