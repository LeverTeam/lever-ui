use crate::types::Color;
use cosmic_text::{Buffer, FontSystem, Metrics, Shaping, SwashCache};
use std::collections::HashMap;

pub struct TextSystem {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    cache: HashMap<(String, u32), TextLayout>,
}

impl TextSystem {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            cache: HashMap::new(),
        }
    }

    pub fn shape(&mut self, text: &str, font_size: f32, color: Color) -> TextLayout {
        let cache_key = (text.to_string(), (font_size * 100.0) as u32);

        if let Some(layout) = self.cache.get(&cache_key) {
            let mut result = layout.clone();
            for glyph in &mut result.glyphs {
                glyph.color = color;
            }
            return result;
        }

        let metrics = Metrics::new(font_size, font_size * 1.2);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        buffer.set_text(
            &mut self.font_system,
            text,
            &cosmic_text::Attrs::new().family(cosmic_text::Family::Name("Arial")),
            Shaping::Advanced,
            None,
        );
        buffer.shape_until_scroll(&mut self.font_system, false);

        let mut glyphs = Vec::new();
        let mut width = 0.0f32;
        let mut cursor_positions = vec![0.0; text.len() + 1];
        let line_height = buffer.metrics().line_height;

        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                glyphs.push(GlyphInstance {
                    glyph_id: glyph.glyph_id as u32,
                    x: glyph.x.round(),
                    y: (run.line_y + glyph.y_offset).round(),
                    color,
                    font_size,
                });
                width = width.max(glyph.x + glyph.w);

                // Map glyph boundaries to character indices
                if glyph.end <= text.len() {
                    cursor_positions[glyph.end] = glyph.x + glyph.w;
                }
                if glyph.start < text.len() {
                    cursor_positions[glyph.start] = glyph.x;
                }
            }
        }

        let layout = TextLayout {
            glyphs,
            width,
            height: line_height,
            cursor_positions,
        };

        self.cache.insert(cache_key, layout.clone());
        layout
    }

    pub fn hit_test(&mut self, text: &str, font_size: f32, x: f32) -> usize {
        let metrics = Metrics::new(font_size, font_size * 1.2);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        buffer.set_text(
            &mut self.font_system,
            text,
            &cosmic_text::Attrs::new().family(cosmic_text::Family::Name("Arial")),
            Shaping::Advanced,
            None,
        );
        buffer.shape_until_scroll(&mut self.font_system, false);

        let cursor = buffer.hit(x, 0.0);
        cursor.map(|c| c.index).unwrap_or(0)
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

#[derive(Debug, Clone)]
pub struct GlyphInstance {
    pub glyph_id: u32,
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub font_size: f32,
}

#[derive(Debug, Clone)]
pub struct TextLayout {
    pub glyphs: Vec<GlyphInstance>,
    pub width: f32,
    pub height: f32,
    pub cursor_positions: Vec<f32>,
}

impl TextLayout {
    pub fn cursor_to_pos(&self, index: usize) -> f32 {
        *self.cursor_positions.get(index).unwrap_or(&self.width)
    }
}
