use crate::tessellation::Tessellator;
use lever_core::types::{Color, Rect};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ColoredVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub uv: [f32; 2],
    pub mode: f32,
    pub size: [f32; 2],
    pub extra: f32,
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
        self.push_gradient_rect(rect, color, color);
    }

    pub fn push_gradient_rect(&mut self, rect: Rect, start_color: Color, end_color: Color) {
        let x1 = rect.x;
        let y1 = rect.y;
        let x2 = rect.x + rect.width;
        let y2 = rect.y + rect.height;

        let c1 = start_color.to_array();
        let c2 = end_color.to_array();
        let start_index = self.vertices.len() as u32;

        self.vertices.push(ColoredVertex {
            position: [x1, y1],
            color: c1,
            uv: [0.0, 0.0],
            mode: 1.0,
            size: [0.0, 0.0],
            extra: 0.0,
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c1,
            uv: [1.0, 0.0],
            mode: 1.0,
            size: [0.0, 0.0],
            extra: 0.0,
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y2],
            color: c2,
            uv: [1.0, 1.0],
            mode: 1.0,
            size: [0.0, 0.0],
            extra: 0.0,
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c2,
            uv: [0.0, 1.0],
            mode: 1.0,
            size: [0.0, 0.0],
            extra: 0.0,
        });

        self.indices.extend_from_slice(&[
            start_index,
            start_index + 1,
            start_index + 2,
            start_index,
            start_index + 2,
            start_index + 3,
        ]);
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
            mode: 0.0,
            size: [0.0, 0.0],
            extra: 0.0,
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c,
            uv: [u2, v1],
            mode: 0.0,
            size: [0.0, 0.0],
            extra: 0.0,
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y2],
            color: c,
            uv: [u2, v2],
            mode: 0.0,
            size: [0.0, 0.0],
            extra: 0.0,
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c,
            uv: [u1, v2],
            mode: 0.0,
            size: [0.0, 0.0],
            extra: 0.0,
        });

        self.indices.extend_from_slice(&[
            start_index,
            start_index + 1,
            start_index + 2,
            start_index,
            start_index + 2,
            start_index + 3,
        ]);
    }

    pub fn push_shadow(&mut self, rect: Rect, radius: f32, color: Color, blur: f32) {
        let expansion = blur * 3.0; // Expand enough to show full blur
        let x1 = rect.x - expansion;
        let y1 = rect.y - expansion;
        let x2 = rect.x + rect.width + expansion;
        let y2 = rect.y + rect.height + expansion;

        let half_w = rect.width / 2.0;
        let half_h = rect.height / 2.0;

        let c = color.to_array();
        let start_index = self.vertices.len() as u32;

        // UV here is pixel distance from center
        self.vertices.push(ColoredVertex {
            position: [x1, y1],
            color: c,
            uv: [-half_w - expansion, -half_h - expansion],
            mode: 2.0,
            size: [rect.width, rect.height],
            extra: blur,
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c,
            uv: [half_w + expansion, -half_h - expansion],
            mode: 2.0,
            size: [rect.width, rect.height],
            extra: blur,
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y2],
            color: c,
            uv: [half_w + expansion, half_h + expansion],
            mode: 2.0,
            size: [rect.width, rect.height],
            extra: blur,
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c,
            uv: [-half_w - expansion, half_h + expansion],
            mode: 2.0,
            size: [rect.width, rect.height],
            extra: blur,
        });

        self.indices.extend_from_slice(&[
            start_index,
            start_index + 1,
            start_index + 2,
            start_index,
            start_index + 2,
            start_index + 3,
        ]);
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

    pub fn push_stroke(&mut self, rect: Rect, radius: f32, thickness: f32, color: Color) {
        self.tess.tessellate_stroke_rect(
            rect,
            radius,
            thickness,
            color,
            &mut self.vertices,
            &mut self.indices,
        );
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
