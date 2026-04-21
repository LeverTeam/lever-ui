use crate::batch::ColoredVertex;
use lever_core::types::{Color, Rect};

pub struct Tessellator {}

impl Tessellator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tessellate_rounded_rect(
        &self,
        rect: Rect,
        radius: f32,
        color: Color,
        vertices: &mut Vec<ColoredVertex>,
        indices: &mut Vec<u32>,
    ) {
        let start_index = vertices.len() as u32;
        let c = color.to_array();

        let half_w = rect.width / 2.0;
        let half_h = rect.height / 2.0;

        vertices.push(ColoredVertex {
            position: [rect.x, rect.y],
            color: c,
            color2: c,
            uv: [-half_w, -half_h],
            mode: 3.0,
            size: [rect.width, rect.height],
            extra: [radius, 0.0, 0.0, 0.0],
        });
        vertices.push(ColoredVertex {
            position: [rect.x + rect.width, rect.y],
            color: c,
            color2: c,
            uv: [half_w, -half_h],
            mode: 3.0,
            size: [rect.width, rect.height],
            extra: [radius, 0.0, 0.0, 0.0],
        });
        vertices.push(ColoredVertex {
            position: [rect.x + rect.width, rect.y + rect.height],
            color: c,
            color2: c,
            uv: [half_w, half_h],
            mode: 3.0,
            size: [rect.width, rect.height],
            extra: [radius, 0.0, 0.0, 0.0],
        });
        vertices.push(ColoredVertex {
            position: [rect.x, rect.y + rect.height],
            color: c,
            color2: c,
            uv: [-half_w, half_h],
            mode: 3.0,
            size: [rect.width, rect.height],
            extra: [radius, 0.0, 0.0, 0.0],
        });

        indices.extend_from_slice(&[
            start_index,
            start_index + 1,
            start_index + 2,
            start_index,
            start_index + 2,
            start_index + 3,
        ]);
    }

    pub fn tessellate_stroke_rect(
        &self,
        rect: Rect,
        radius: f32,
        thickness: f32,
        color: Color,
        vertices: &mut Vec<ColoredVertex>,
        indices: &mut Vec<u32>,
    ) {
        let start_index = vertices.len() as u32;
        let c = color.to_array();
        let half_w = rect.width / 2.0;
        let half_h = rect.height / 2.0;

        vertices.push(ColoredVertex {
            position: [rect.x - thickness, rect.y - thickness],
            color: c,
            color2: c,
            uv: [-half_w - thickness, -half_h - thickness],
            mode: 4.0,
            size: [rect.width, rect.height],
            extra: [radius, thickness, 0.0, 0.0],
        });
        vertices.push(ColoredVertex {
            position: [rect.x + rect.width + thickness, rect.y - thickness],
            color: c,
            color2: c,
            uv: [half_w + thickness, -half_h - thickness],
            mode: 4.0,
            size: [rect.width, rect.height],
            extra: [radius, thickness, 0.0, 0.0],
        });
        vertices.push(ColoredVertex {
            position: [
                rect.x + rect.width + thickness,
                rect.y + rect.height + thickness,
            ],
            color: c,
            color2: c,
            uv: [half_w + thickness, half_h + thickness],
            mode: 4.0,
            size: [rect.width, rect.height],
            extra: [radius, thickness, 0.0, 0.0],
        });
        vertices.push(ColoredVertex {
            position: [rect.x - thickness, rect.y + rect.height + thickness],
            color: c,
            color2: c,
            uv: [-half_w - thickness, half_h + thickness],
            mode: 4.0,
            size: [rect.width, rect.height],
            extra: [radius, thickness, 0.0, 0.0],
        });

        indices.extend_from_slice(&[
            start_index,
            start_index + 1,
            start_index + 2,
            start_index,
            start_index + 2,
            start_index + 3,
        ]);
    }
}
