use crate::types::{Color, Rect};

#[derive(Debug, Clone)]
pub enum DrawCommand {
    RoundedRect {
        rect: Rect,
        color: Color,
        radius: f32,
        shadow: Option<crate::types::BoxShadow>,
    },
    GradientRect {
        rect: Rect,
        gradient: crate::types::Gradient,
        radius: f32,
    },
    ClipPush(Rect),
    ClipPop,
    Text {
        pos: crate::types::Point,
        glyphs: Vec<crate::text::GlyphInstance>,
    },
    Stroke {
        rect: Rect,
        color: Color,
        radius: f32,
        thickness: f32,
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
        self.commands.push(DrawCommand::RoundedRect {
            rect,
            color,
            radius: corner_radius,
            shadow: None,
        });
    }

    pub fn rounded_rect(&mut self, rect: Rect, color: Color, radius: f32) {
        self.commands.push(DrawCommand::RoundedRect {
            rect,
            color,
            radius,
            shadow: None,
        });
    }

    pub fn shadowed_rect(
        &mut self,
        rect: Rect,
        color: Color,
        radius: f32,
        shadow: crate::types::BoxShadow,
    ) {
        self.commands.push(DrawCommand::RoundedRect {
            rect,
            color,
            radius,
            shadow: Some(shadow),
        });
    }

    pub fn gradient_rect(&mut self, rect: Rect, gradient: crate::types::Gradient, radius: f32) {
        self.commands.push(DrawCommand::GradientRect {
            rect,
            gradient,
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

    pub fn stroke_rect(&mut self, rect: Rect, color: Color, radius: f32, thickness: f32) {
        self.commands.push(DrawCommand::Stroke {
            rect,
            color,
            radius,
            thickness,
        });
    }

    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    pub fn commands_mut(&mut self) -> &mut Vec<DrawCommand> {
        &mut self.commands
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}
