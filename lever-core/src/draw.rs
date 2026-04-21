use crate::types::{Color, Rect};

#[derive(Debug, Clone)]
pub enum DrawCommand {
    ColoredRect {
        rect: Rect,
        color: Color,
        corner_radius: f32,
    },
    RoundedRect {
        rect: Rect,
        color: Color,
        radius: f32,
    },
    ClipPush(Rect),
    ClipPop,
    Text {
        pos: crate::types::Point,
        glyphs: Vec<crate::text::GlyphInstance>,
    },
}

pub struct DrawList {
    commands: Vec<DrawCommand>,
}

impl DrawList {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn colored_rect(&mut self, rect: Rect, color: Color, corner_radius: f32) {
        self.commands.push(DrawCommand::ColoredRect {
            rect,
            color,
            corner_radius,
        });
    }

    pub fn rounded_rect(&mut self, rect: Rect, color: Color, radius: f32) {
        self.commands.push(DrawCommand::RoundedRect {
            rect,
            color,
            radius,
        });
    }

    pub fn clip_push(&mut self, rect: Rect) {
        self.commands.push(DrawCommand::ClipPush(rect));
    }

    pub fn clip_pop(&mut self) {
        self.commands.push(DrawCommand::ClipPop);
    }

    pub fn text(&mut self, pos: crate::types::Point, glyphs: Vec<crate::text::GlyphInstance>) {
        self.commands.push(DrawCommand::Text { pos, glyphs });
    }

    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}
