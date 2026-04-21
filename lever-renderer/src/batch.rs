use lever_core::types::{Color, Rect};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ColoredVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

pub struct RectBatch {
    vertices: Vec<ColoredVertex>,
}

impl RectBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn push_rect(&mut self, rect: Rect, color: Color) {
        let x1 = rect.x;
        let y1 = rect.y;
        let x2 = rect.x + rect.width;
        let y2 = rect.y + rect.height;

        let c = color.to_array();

        // Triangle 1
        self.vertices.push(ColoredVertex {
            position: [x1, y1],
            color: c,
        });
        self.vertices.push(ColoredVertex {
            position: [x2, y1],
            color: c,
        });
        self.vertices.push(ColoredVertex {
            position: [x1, y2],
            color: c,
        });

        // Triangle 2
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
    }

    pub fn vertices(&self) -> &[ColoredVertex] {
        &self.vertices
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }
}
