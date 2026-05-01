use crate::types::Color;
use fontdue::Font;
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use std::collections::HashMap;
use std::sync::Arc;

pub struct TextSystem {
    fonts: Arc<Vec<Font>>,
    cache: HashMap<CacheKey, TextLayout>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    text: String,
    font_size: u32,
    max_width: u32,
    align: crate::types::TextAlign,
}

impl TextSystem {
    pub fn new() -> Self {
        let font_data = std::fs::read("C:\\Windows\\Fonts\\segoeui.ttf")
            .or_else(|_| std::fs::read("C:\\Windows\\Fonts\\arial.ttf"))
            .expect("Failed to load system font");

        let font = Font::from_bytes(font_data, fontdue::FontSettings::default())
            .expect("Failed to parse font");

        Self {
            fonts: Arc::new(vec![font]),
            cache: HashMap::new(),
        }
    }

    pub fn fonts(&self) -> Arc<Vec<Font>> {
        self.fonts.clone()
    }

    pub fn shape(
        &mut self,
        text: &str,
        font_size: f32,
        color: Color,
        max_width: Option<f32>,
        align: crate::types::TextAlign,
    ) -> TextLayout {
        let max_width = max_width.filter(|w| w.is_finite());

        let px_size = font_size.round();

        let cache_key = CacheKey {
            text: text.to_string(),
            font_size: px_size as u32,
            max_width: max_width.map(|w| (w * 100.0) as u32).unwrap_or(u32::MAX),
            align,
        };

        if let Some(layout) = self.cache.get(&cache_key) {
            let mut result = layout.clone();
            for glyph in &mut result.glyphs {
                glyph.color = color;
            }
            return result;
        }

        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            max_width,
            horizontal_align: match align {
                crate::types::TextAlign::Left => fontdue::layout::HorizontalAlign::Left,
                crate::types::TextAlign::Center => fontdue::layout::HorizontalAlign::Center,
                crate::types::TextAlign::Right => fontdue::layout::HorizontalAlign::Right,
            },
            ..LayoutSettings::default()
        });
        layout.append(&self.fonts, &TextStyle::new(text, px_size, 0));

        let glyphs_raw = layout.glyphs();
        let mut glyphs = Vec::with_capacity(glyphs_raw.len());
        let mut width = 0.0f32;
        let mut cursor_positions = vec![0.0f32; text.len() + 1];

        for g in glyphs_raw {
            glyphs.push(GlyphInstance {
                glyph_id: g.key.glyph_index as u32,
                x: g.x,
                y: g.y,
                color,
                font_size: px_size,
            });

            width = width.max(g.x + g.width as f32);

            if g.byte_offset < cursor_positions.len() {
                cursor_positions[g.byte_offset] = g.x;

                let char_len = text[g.byte_offset..]
                    .chars()
                    .next()
                    .map(|c| c.len_utf8())
                    .unwrap_or(1);
                let next_idx = g.byte_offset + char_len;
                if next_idx < cursor_positions.len() {
                    cursor_positions[next_idx] = g.x + g.width as f32;
                }
            }
        }

        let (height, vertical_shift) = layout
            .lines()
            .map(|lines| {
                if lines.is_empty() {
                    (px_size * 1.2, 0.0)
                } else {
                    let last = &lines[lines.len() - 1];
                    let total_height = last.baseline_y - last.min_descent;

                    let first = &lines[0];
                    let visual_center =
                        (first.baseline_y + (first.baseline_y - px_size * 0.7)) / 2.0;
                    let layout_center = total_height / 2.0;
                    let shift = layout_center - visual_center;

                    (total_height, shift)
                }
            })
            .unwrap_or((px_size * 1.2, 0.0));

        for glyph in &mut glyphs {
            glyph.y += vertical_shift;
        }

        let result = TextLayout {
            glyphs,
            width: width.ceil(),
            height: height.ceil(),
            cursor_positions,
        };

        self.cache.insert(cache_key, result.clone());
        result
    }

    pub fn hit_test(&mut self, text: &str, font_size: f32, x: f32) -> usize {
        let layout = self.shape(
            text,
            font_size,
            Color::WHITE,
            None,
            crate::types::TextAlign::Left,
        );
        let mut best = 0;
        let mut best_dist = f32::MAX;
        for (i, &pos) in layout.cursor_positions.iter().enumerate() {
            let dist = (pos - x).abs();
            if dist < best_dist {
                best_dist = dist;
                best = i;
            }
        }
        best
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
