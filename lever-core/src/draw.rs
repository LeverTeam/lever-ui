use crate::types::{Color, Point, Rect};

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

    Image {
        rect: Rect,
        texture: crate::types::TextureId,
        tint: Color,
        uv: [f32; 4],
    },

    PushOpacity(f32),

    PopOpacity,

    PushTranslation(Point),

    PopTranslation,

    Triangle {
        p1: Point,
        p2: Point,
        p3: Point,
        color: Color,
    },

    PushScale {
        scale: f32,
        pivot: Point,
    },

    PopScale,

    Arc {
        rect: Rect,
        color: Color,
        thickness: f32,
        progress: f32,
    },
}

pub struct DrawList {
    commands: Vec<DrawCommand>,
    deferred_commands: Vec<DrawCommand>,
    clip_stack: Vec<Rect>,
}

impl DrawList {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            deferred_commands: Vec::new(),
            clip_stack: Vec::new(),
        }
    }

    pub fn current_clip(&self) -> Option<Rect> {
        self.clip_stack.last().copied()
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
        let new_clip = if let Some(last) = self.clip_stack.last() {
            last.intersect(rect).unwrap_or(Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            })
        } else {
            rect
        };
        self.clip_stack.push(new_clip);
        self.commands.push(DrawCommand::ClipPush(rect));
    }

    pub fn clip_pop(&mut self) {
        self.clip_stack.pop();
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

    pub fn textured_rect(
        &mut self,
        rect: Rect,
        texture: crate::types::TextureId,
        tint: Color,
        uv: [f32; 4],
    ) {
        self.commands.push(DrawCommand::Image {
            rect,
            texture,
            tint,
            uv,
        });
    }

    pub fn push_opacity(&mut self, opacity: f32) {
        self.commands.push(DrawCommand::PushOpacity(opacity));
    }

    pub fn pop_opacity(&mut self) {
        self.commands.push(DrawCommand::PopOpacity);
    }

    pub fn push_translation(&mut self, offset: Point) {
        self.commands.push(DrawCommand::PushTranslation(offset));
    }

    pub fn pop_translation(&mut self) {
        self.commands.push(DrawCommand::PopTranslation);
    }

    pub fn push_scale(&mut self, scale: f32, pivot: Point) {
        self.commands.push(DrawCommand::PushScale { scale, pivot });
    }

    pub fn pop_scale(&mut self) {
        self.commands.push(DrawCommand::PopScale);
    }

    pub fn triangle(&mut self, p1: Point, p2: Point, p3: Point, color: Color) {
        self.commands
            .push(DrawCommand::Triangle { p1, p2, p3, color });
    }

    pub fn arc(&mut self, rect: Rect, color: Color, thickness: f32, progress: f32) {
        self.commands.push(DrawCommand::Arc {
            rect,
            color,
            thickness,
            progress,
        });
    }

    pub fn line(&mut self, p1: Point, p2: Point, thickness: f32, color: Color) {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 0.001 {
            return;
        }

        let nx = -dy / len * (thickness / 2.0);
        let ny = dx / len * (thickness / 2.0);

        let q1 = Point {
            x: p1.x + nx,
            y: p1.y + ny,
        };
        let q2 = Point {
            x: p1.x - nx,
            y: p1.y - ny,
        };
        let q3 = Point {
            x: p2.x + nx,
            y: p2.y + ny,
        };
        let q4 = Point {
            x: p2.x - nx,
            y: p2.y - ny,
        };

        self.triangle(q1, q2, q3, color);
        self.triangle(q2, q3, q4, color);
    }

    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    pub fn commands_mut(&mut self) -> &mut Vec<DrawCommand> {
        &mut self.commands
    }

    pub fn deferred_commands(&self) -> &[DrawCommand] {
        &self.deferred_commands
    }

    pub fn push_deferred(&mut self, command: DrawCommand) {
        self.deferred_commands.push(command);
    }

    pub fn clear(&mut self) {
        self.commands.clear();
        self.deferred_commands.clear();
        self.clip_stack.clear();
    }
}
