use crate::batch::{ColoredVertex, RectBatch};
use crate::error::RendererError;
use crate::shader::{compile_shader, link_program};
use glow::{Context, HasContext};
use lever_core::draw::{DrawCommand, DrawList};
use lever_core::types::{Color, Size};
use std::sync::Arc;

const VERT_SHADER_SOURCE: &str = r#"
#version 330 core
layout(location = 0) in vec2 a_position;
layout(location = 1) in vec4 a_color;
layout(location = 2) in vec2 a_uv;
uniform vec2 u_viewport;
out vec4 v_color;
out vec2 v_uv;
void main() {
    vec2 ndc = (a_position / u_viewport) * 2.0 - 1.0;
    ndc.y = -ndc.y;
    gl_Position = vec4(ndc, 0.0, 1.0);
    v_color = a_color;
    v_uv = a_uv;
}
"#;

const FRAG_SHADER_SOURCE: &str = r#"
#version 330 core
in vec4 v_color;
in vec2 v_uv;
uniform sampler2D u_texture;
out vec4 frag_color;
void main() {
    float alpha = texture(u_texture, v_uv).r;
    frag_color = vec4(v_color.rgb, v_color.a * alpha);
}
"#;

pub struct Renderer {
    gl: Arc<Context>,
    rect_program: glow::Program,
    rect_vao: glow::VertexArray,
    rect_vbo: glow::Buffer,
    rect_ibo: glow::Buffer,
    u_viewport: glow::UniformLocation,
    u_texture: glow::UniformLocation,
    batch: RectBatch,
    viewport_size: Size,
    atlas: crate::atlas::GlyphAtlas,
    font: fontdue::Font,
}

impl Renderer {
    pub fn new(gl: Arc<Context>) -> Result<Self, RendererError> {
        unsafe {
            let vert = compile_shader(&gl, glow::VERTEX_SHADER, VERT_SHADER_SOURCE)?;
            let frag = compile_shader(&gl, glow::FRAGMENT_SHADER, FRAG_SHADER_SOURCE)?;
            let program = link_program(&gl, vert, frag)?;

            gl.delete_shader(vert);
            gl.delete_shader(frag);

            let u_viewport = gl
                .get_uniform_location(program, "u_viewport")
                .ok_or(RendererError::GlAllocation("Viewport Uniform"))?;

            let u_texture = gl
                .get_uniform_location(program, "u_texture")
                .ok_or(RendererError::GlAllocation("Texture Uniform"))?;

            let vao = gl
                .create_vertex_array()
                .map_err(|_| RendererError::GlAllocation("VAO"))?;
            let vbo = gl
                .create_buffer()
                .map_err(|_| RendererError::GlAllocation("VBO"))?;
            let ibo = gl
                .create_buffer()
                .map_err(|_| RendererError::GlAllocation("IBO"))?;

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let stride = std::mem::size_of::<ColoredVertex>() as i32;
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, stride, 8);
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, stride, 24);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));

            gl.bind_vertex_array(None);

            let atlas = crate::atlas::GlyphAtlas::new(gl.clone(), 1024, 1024);

            let font_data = std::fs::read("C:\\Windows\\Fonts\\arial.ttf")
                .or_else(|_| std::fs::read("C:\\Windows\\Fonts\\segoeui.ttf"))
                .map_err(|_| RendererError::GlAllocation("System Font"))?;
            let font = fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default())
                .map_err(|_| RendererError::GlAllocation("Font Parsing"))?;

            Ok(Self {
                gl,
                rect_program: program,
                rect_vao: vao,
                rect_vbo: vbo,
                rect_ibo: ibo,
                u_viewport,
                u_texture,
                batch: RectBatch::new(),
                viewport_size: Size {
                    width: 0.0,
                    height: 0.0,
                },
                atlas,
                font,
            })
        }
    }

    pub fn begin_frame(&mut self, viewport: Size, clear_color: Color) {
        self.viewport_size = viewport;
        unsafe {
            self.gl
                .viewport(0, 0, viewport.width as i32, viewport.height as i32);
            self.gl
                .clear_color(clear_color.r, clear_color.g, clear_color.b, clear_color.a);
            self.gl.clear(glow::COLOR_BUFFER_BIT);

            self.gl.use_program(Some(self.rect_program));
            self.gl
                .uniform_2_f32(Some(&self.u_viewport), viewport.width, viewport.height);

            self.gl.active_texture(glow::TEXTURE0);
            self.gl
                .bind_texture(glow::TEXTURE_2D, Some(self.atlas.texture()));
            self.gl.uniform_1_i32(Some(&self.u_texture), 0);

            self.gl.enable(glow::BLEND);
            self.gl
                .blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn render(&mut self, draw_list: &DrawList) {
        self.batch.clear();

        for command in draw_list.commands() {
            match command {
                DrawCommand::ColoredRect { rect, color, .. } => {
                    self.batch.push_rect(*rect, *color);
                }
                DrawCommand::RoundedRect {
                    rect,
                    color,
                    radius,
                } => {
                    self.batch.push_rounded_rect(*rect, *radius, *color);
                }
                DrawCommand::Text { pos, glyphs } => {
                    for glyph in glyphs {
                        let config = fontdue::layout::GlyphRasterConfig {
                            glyph_index: glyph.glyph_id as u16,
                            px: glyph.font_size,
                            font_hash: self.font.file_hash(),
                        };

                        if let Some(region) = self.atlas.get_or_insert(&self.font, config) {
                            let (atlas_w, atlas_h) = self.atlas.size();
                            let uv_rect = [
                                region.x as f32 / atlas_w as f32,
                                region.y as f32 / atlas_h as f32,
                                region.width as f32 / atlas_w as f32,
                                region.height as f32 / atlas_h as f32,
                            ];

                            let glyph_rect = lever_core::types::Rect {
                                x: pos.x + glyph.x + region.metrics_x as f32,
                                y: pos.y + glyph.y - region.metrics_y as f32,
                                width: region.width as f32,
                                height: region.height as f32,
                            };

                            self.batch
                                .push_textured_rect(glyph_rect, glyph.color, uv_rect);
                        }
                    }
                }
                DrawCommand::ClipPush(rect) => {
                    self.flush();
                    unsafe {
                        self.gl.enable(glow::SCISSOR_TEST);
                        let y = self.viewport_size.height - (rect.y + rect.height);
                        self.gl.scissor(
                            rect.x as i32,
                            y as i32,
                            rect.width as i32,
                            rect.height as i32,
                        );
                    }
                }
                DrawCommand::ClipPop => {
                    self.flush();
                    unsafe {
                        self.gl.disable(glow::SCISSOR_TEST);
                    }
                }
            }
        }

        self.flush();
    }

    fn flush(&mut self) {
        let vertices = self.batch.vertices();
        let indices = self.batch.indices();
        if vertices.is_empty() || indices.is_empty() {
            return;
        }

        unsafe {
            self.gl.bind_vertex_array(Some(self.rect_vao));

            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.rect_vbo));
            let v_data: &[u8] = std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<ColoredVertex>(),
            );
            self.gl
                .buffer_data_u8_slice(glow::ARRAY_BUFFER, v_data, glow::DYNAMIC_DRAW);

            self.gl
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.rect_ibo));
            let i_data: &[u8] = std::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * std::mem::size_of::<u32>(),
            );
            self.gl
                .buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, i_data, glow::DYNAMIC_DRAW);

            self.gl
                .draw_elements(glow::TRIANGLES, indices.len() as i32, glow::UNSIGNED_INT, 0);

            self.gl.bind_vertex_array(None);
        }

        self.batch.clear();
    }

    pub fn end_frame(&mut self) {}
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.rect_program);
            self.gl.delete_vertex_array(self.rect_vao);
            self.gl.delete_buffer(self.rect_vbo);
            self.gl.delete_buffer(self.rect_ibo);
        }
    }
}
