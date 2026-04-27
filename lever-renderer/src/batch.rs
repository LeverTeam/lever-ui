use lever_core::types::{Color, Point, Rect};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColoredVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub color2: [f32; 4],
    pub uv: [f32; 2],
    pub mode: f32,
    pub size: [f32; 2],
    pub extra: [f32; 4],
}

pub struct RectBatch {
    vertices: Vec<ColoredVertex>,
    indices: Vec<u32>,
}

impl RectBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn push_rect(&mut self, rect: Rect, color: Color) {
        self.push_rounded_rect(rect, 0.0, color);
    }

    pub fn push_gradient_rect(&mut self, rect: Rect, start_color: Color, end_color: Color) {
        self.push_rounded_gradient_rect(rect, 0.0, start_color, end_color);
    }

    fn snap_rect(rect: Rect, margin: f32) -> (f32, f32, f32, f32, f32, f32) {
        let cx = (rect.x + rect.width / 2.0).round();
        let cy = (rect.y + rect.height / 2.0).round();
        let half_w = (rect.width / 2.0).round();
        let half_h = (rect.height / 2.0).round();

        let x1 = cx - half_w - margin;
        let y1 = cy - half_h - margin;
        let x2 = cx + half_w + margin;
        let y2 = cy + half_h + margin;

        (x1, y1, x2, y2, half_w * 2.0, half_h * 2.0)
    }

    pub fn push_rounded_rect(&mut self, rect: Rect, radius: f32, color: Color) {
        let margin = 2.0;
        let (x1, y1, x2, y2, w, h) = Self::snap_rect(rect, margin);

        let hw = w / 2.0;
        let hh = h / 2.0;

        let c = color.to_array();
        let start_index = self.vertices.len() as u32;

        let u_min = -hw - margin;
        let v_min = -hh - margin;
        let u_max = hw + margin;
        let v_max = hh + margin;

        self.vertices.push(ColoredVertex {
            position: [x1, y1],
            color: c,
            color2: c,
            uv: [u_min, v_min],
            mode: 1.0,
            size: [w, h],
            extra: [radius, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c,
            color2: c,
            uv: [u_max, v_min],
            mode: 1.0,
            size: [w, h],
            extra: [radius, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y2],
            color: c,
            color2: c,
            uv: [u_max, v_max],
            mode: 1.0,
            size: [w, h],
            extra: [radius, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c,
            color2: c,
            uv: [u_min, v_max],
            mode: 1.0,
            size: [w, h],
            extra: [radius, 0.0, 0.0, 0.0],
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

    pub fn push_rounded_gradient_rect(
        &mut self,
        rect: Rect,
        radius: f32,
        start_color: Color,
        end_color: Color,
    ) {
        let margin = 2.0;
        let (x1, y1, x2, y2, w, h) = Self::snap_rect(rect, margin);

        let hw = w / 2.0;
        let hh = h / 2.0;

        let c1 = start_color.to_array();
        let c2 = end_color.to_array();
        let start_index = self.vertices.len() as u32;

        let u_min = -hw - margin;
        let v_min = -hh - margin;
        let u_max = hw + margin;
        let v_max = hh + margin;

        self.vertices.push(ColoredVertex {
            position: [x1, y1],
            color: c1,
            color2: c2,
            uv: [u_min, v_min],
            mode: 3.0,
            size: [w, h],
            extra: [radius, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c1,
            color2: c2,
            uv: [u_max, v_min],
            mode: 3.0,
            size: [w, h],
            extra: [radius, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y2],
            color: c1,
            color2: c2,
            uv: [u_max, v_max],
            mode: 3.0,
            size: [w, h],
            extra: [radius, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c1,
            color2: c2,
            uv: [u_min, v_max],
            mode: 3.0,
            size: [w, h],
            extra: [radius, 0.0, 0.0, 0.0],
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

    pub fn push_textured_rect(&mut self, rect: Rect, color: Color, uv_rect: [f32; 4], mode: f32) {
        let x1 = rect.x.round();
        let y1 = rect.y.round();
        let x2 = (rect.x + rect.width).round();
        let y2 = (rect.y + rect.height).round();
        let w = x2 - x1;
        let h = y2 - y1;

        let u1 = uv_rect[0];
        let v1 = uv_rect[1];
        let u2 = uv_rect[0] + uv_rect[2];
        let v2 = uv_rect[1] + uv_rect[3];

        let c = color.to_array();
        let start_index = self.vertices.len() as u32;

        self.vertices.push(ColoredVertex {
            position: [x1, y1],
            color: c,
            color2: c,
            uv: [u1, v1],
            mode,
            size: [w, h],
            extra: [0.0, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c,
            color2: c,
            uv: [u2, v1],
            mode,
            size: [w, h],
            extra: [0.0, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y2],
            color: c,
            color2: c,
            uv: [u2, v2],
            mode,
            size: [w, h],
            extra: [0.0, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c,
            color2: c,
            uv: [u1, v2],
            mode,
            size: [w, h],
            extra: [0.0, 0.0, 0.0, 0.0],
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

    pub fn push_glyph_rect(&mut self, rect: Rect, uv_rect: [f32; 4], color: Color) {
        self.push_textured_rect(rect, color, uv_rect, 0.0);
    }

    pub fn push_image_rect(&mut self, rect: Rect, uv_rect: [f32; 4], color: Color) {
        self.push_textured_rect(rect, color, uv_rect, 4.0);
    }

    pub fn push_shadow(&mut self, rect: Rect, radius: f32, color: Color, blur: f32) {
        let expansion = blur * 3.0;
        let (x1, y1, x2, y2, w, h) = Self::snap_rect(rect, expansion);

        let hw = w / 2.0;
        let hh = h / 2.0;

        let c = color.to_array();
        let start_index = self.vertices.len() as u32;

        let u_min = -hw - expansion;
        let v_min = -hh - expansion;
        let u_max = hw + expansion;
        let v_max = hh + expansion;

        self.vertices.push(ColoredVertex {
            position: [x1, y1],
            color: c,
            color2: c,
            uv: [u_min, v_min],
            mode: 2.0,
            size: [w, h],
            extra: [radius, blur, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c,
            color2: c,
            uv: [u_max, v_min],
            mode: 2.0,
            size: [w, h],
            extra: [radius, blur, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y2],
            color: c,
            color2: c,
            uv: [u_max, v_max],
            mode: 2.0,
            size: [w, h],
            extra: [radius, blur, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c,
            color2: c,
            uv: [u_min, v_max],
            mode: 2.0,
            size: [w, h],
            extra: [radius, blur, 0.0, 0.0],
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

    pub fn push_stroke(&mut self, rect: Rect, radius: f32, thickness: f32, color: Color) {
        let margin = 2.0 + thickness;
        let (x1, y1, x2, y2, w, h) = Self::snap_rect(rect, margin);

        let hw = w / 2.0;
        let hh = h / 2.0;

        let c = color.to_array();
        let start_index = self.vertices.len() as u32;

        let u_min = -hw - margin;
        let v_min = -hh - margin;
        let u_max = hw + margin;
        let v_max = hh + margin;

        self.vertices.push(ColoredVertex {
            position: [x1, y1],
            color: c,
            color2: c,
            uv: [u_min, v_min],
            mode: 6.0,
            size: [w, h],
            extra: [radius, thickness, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c,
            color2: c,
            uv: [u_max, v_min],
            mode: 6.0,
            size: [w, h],
            extra: [radius, thickness, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y2],
            color: c,
            color2: c,
            uv: [u_max, v_max],
            mode: 6.0,
            size: [w, h],
            extra: [radius, thickness, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c,
            color2: c,
            uv: [u_min, v_max],
            mode: 6.0,
            size: [w, h],
            extra: [radius, thickness, 0.0, 0.0],
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

    pub fn push_triangle(&mut self, p1: Point, p2: Point, p3: Point, color: Color) {
        let c = color.to_array();
        let start_index = self.vertices.len() as u32;

        self.vertices.push(ColoredVertex {
            position: [p1.x, p1.y],
            color: c,
            color2: c,
            uv: [0.0, 0.0],
            mode: 7.0,
            size: [0.0, 0.0],
            extra: [0.0, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [p2.x, p2.y],
            color: c,
            color2: c,
            uv: [0.0, 0.0],
            mode: 7.0,
            size: [0.0, 0.0],
            extra: [0.0, 0.0, 0.0, 0.0],
        });
        self.vertices.push(ColoredVertex {
            position: [p3.x, p3.y],
            color: c,
            color2: c,
            uv: [0.0, 0.0],
            mode: 7.0,
            size: [0.0, 0.0],
            extra: [0.0, 0.0, 0.0, 0.0],
        });

        self.indices
            .extend_from_slice(&[start_index, start_index + 1, start_index + 2]);
    }

    pub fn vertices(&self) -> &[ColoredVertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}
