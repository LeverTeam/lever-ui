use crate::tessellation::Tessellator;
use lever_core::types::{Color, Rect};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ColoredVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub uv: [f32; 2],
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
        let uv = 0.5 / 1024.0;
        self.push_textured_rect(rect, color, [uv, uv, 0.0, 0.0]);
    }

    pub fn push_textured_rect(&mut self, rect: Rect, color: Color, uv_rect: [f32; 4]) {
        let x1 = rect.x;
        let y1 = rect.y;
        let x2 = rect.x + rect.width;
        let y2 = rect.y + rect.height;

        let u1 = uv_rect[0];
        let v1 = uv_rect[1];
        let u2 = uv_rect[0] + uv_rect[2];
        let v2 = uv_rect[1] + uv_rect[3];

        let c = color.to_array();
        let start_index = self.vertices.len() as u32;

        self.vertices.push(ColoredVertex {
            position: [x1, y1],
            color: c,
            uv: [u1, v1],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c,
            uv: [u2, v1],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y2],
            color: c,
            uv: [u2, v2],
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c,
            uv: [u1, v2],
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
