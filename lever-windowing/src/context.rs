use glutin::context::PossiblyCurrentContext;
use glutin::display::Display;
use glutin::surface::{GlSurface, Surface, WindowSurface};
use std::num::NonZeroU32;

pub struct GlContext {
    pub context: PossiblyCurrentContext,
    pub surface: Surface<WindowSurface>,
    pub display: Display,
}

impl GlContext {
    pub fn swap_buffers(&self) -> Result<(), glutin::error::Error> {
        self.surface.swap_buffers(&self.context)
    }

    pub fn resize(&self, width: NonZeroU32, height: NonZeroU32) {
        self.surface.resize(&self.context, width, height);
    }
}
