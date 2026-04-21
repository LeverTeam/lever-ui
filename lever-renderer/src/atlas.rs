use fontdue::layout::GlyphRasterConfig;
use fontdue::Font;
use glow::HasContext;
use guillotiere::{size2, AtlasAllocator};
use std::collections::HashMap;
use std::sync::Arc;

pub struct GlyphAtlas {
    gl: Arc<glow::Context>,
    texture: glow::Texture,
    allocator: AtlasAllocator,
    cache: HashMap<GlyphRasterConfig, AtlasRegion>,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct AtlasRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub metrics_x: i32,
    pub metrics_y: i32,
}

impl GlyphAtlas {
    pub fn new(gl: Arc<glow::Context>, width: u32, height: u32) -> Self {
        let texture = unsafe {
            let tex = gl.create_texture().expect("Failed to create atlas texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            let data = vec![0u8; (width * height) as usize];
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RED as i32,
                width as i32,
                height as i32,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                Some(&data),
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );

            let white_pixel = [255u8];
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                1,
                1,
                glow::RED,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(&white_pixel),
            );

            tex
        };

        let mut allocator = AtlasAllocator::new(size2(width as i32, height as i32));
        let _ = allocator.allocate(size2(1, 1));

        Self {
            gl,
            texture,
            allocator,
            cache: HashMap::new(),
            width,
            height,
        }
    }

    pub fn get_or_insert(&mut self, font: &Font, config: GlyphRasterConfig) -> Option<AtlasRegion> {
        if let Some(region) = self.cache.get(&config) {
            return Some(*region);
        }

        let (metrics, bitmap) = font.rasterize_config(config);
        if metrics.width == 0 || metrics.height == 0 {
            return None;
        }

        let allocation = self
            .allocator
            .allocate(size2(metrics.width as i32 + 2, metrics.height as i32 + 2))?;
        let region = AtlasRegion {
            x: (allocation.rectangle.min.x + 1) as u32,
            y: (allocation.rectangle.min.y + 1) as u32,
            width: metrics.width as u32,
            height: metrics.height as u32,
            metrics_x: metrics.xmin,
            metrics_y: metrics.ymin + metrics.height as i32,
        };

        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            self.gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
            self.gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                region.x as i32,
                region.y as i32,
                region.width as i32,
                region.height as i32,
                glow::RED,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(&bitmap),
            );
        }

        self.cache.insert(config, region);
        Some(region)
    }

    pub fn texture(&self) -> glow::Texture {
        self.texture
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Drop for GlyphAtlas {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.texture);
        }
    }
}
