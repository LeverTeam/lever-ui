use crate::batch::ColoredVertex;
use lever_core::types::{Color, Rect};
use lyon::math::Box2D;
use lyon::tessellation::*;

pub struct Tessellator {
    fill_tess: FillTessellator,
    stroke_tess: StrokeTessellator,
}

impl Tessellator {
    pub fn new() -> Self {
        Self {
            fill_tess: FillTessellator::new(),
            stroke_tess: StrokeTessellator::new(),
        }
    }

    pub fn tessellate_rounded_rect(
        &mut self,
        rect: Rect,
        radius: f32,
        color: Color,
        vertices: &mut Vec<ColoredVertex>,
        indices: &mut Vec<u32>,
    ) {
        let start_index = vertices.len() as u32;
        let c = color.to_array();

        let mut builder = lyon::path::Path::builder();
        builder.add_rounded_rectangle(
            &Box2D::new(
                lyon::math::point(rect.x, rect.y),
                lyon::math::point(rect.x + rect.width, rect.y + rect.height),
            ),
            &lyon::path::builder::BorderRadii::new(radius),
            lyon::path::Winding::Positive,
        );
        let path = builder.build();

        let mut v_buffers: VertexBuffers<ColoredVertex, u32> = VertexBuffers::new();

        let uv = 0.5 / 1024.0;
        let mut geometry_builder =
            BuffersBuilder::new(&mut v_buffers, |vertex: FillVertex| ColoredVertex {
                position: [vertex.position().x, vertex.position().y],
                color: c,
                uv: [uv, uv],
                mode: 1.0,
                size: [rect.width, rect.height],
                extra: radius,
            });

        let _ =
            self.fill_tess
                .tessellate_path(&path, &FillOptions::default(), &mut geometry_builder);

        vertices.extend(v_buffers.vertices);
        for idx in v_buffers.indices {
            indices.push(start_index + idx);
        }
    }

    pub fn tessellate_stroke_rect(
        &mut self,
        rect: Rect,
        radius: f32,
        thickness: f32,
        color: Color,
        vertices: &mut Vec<ColoredVertex>,
        indices: &mut Vec<u32>,
    ) {
        let start_index = vertices.len() as u32;
        let c = color.to_array();

        let mut builder = lyon::path::Path::builder();
        builder.add_rounded_rectangle(
            &Box2D::new(
                lyon::math::point(rect.x, rect.y),
                lyon::math::point(rect.x + rect.width, rect.y + rect.height),
            ),
            &lyon::path::builder::BorderRadii::new(radius),
            lyon::path::Winding::Positive,
        );
        let path = builder.build();

        let mut v_buffers: VertexBuffers<ColoredVertex, u32> = VertexBuffers::new();

        let uv = 0.5 / 1024.0;
        let mut geometry_builder =
            BuffersBuilder::new(&mut v_buffers, |vertex: StrokeVertex| ColoredVertex {
                position: [vertex.position().x, vertex.position().y],
                color: c,
                uv: [uv, uv],
                mode: 1.0,
                size: [rect.width, rect.height],
                extra: radius,
            });

        let _ = self.stroke_tess.tessellate_path(
            &path,
            &StrokeOptions::default().with_line_width(thickness),
            &mut geometry_builder,
        );

        vertices.extend(v_buffers.vertices);
        for idx in v_buffers.indices {
            indices.push(start_index + idx);
        }
    }
}
