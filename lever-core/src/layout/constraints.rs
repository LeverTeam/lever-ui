use crate::types::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Anchor {
    Top,
    Bottom,
    Left,
    Right,
    CenterX,
    CenterY,
    Width,
    Height,
    AspectRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Parent,
    Child(usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constraint {
    pub anchor: Anchor,
    pub target: Target,
    pub target_anchor: Anchor,
    pub offset: f32,
}

#[derive(Debug, Clone, Default)]
pub struct ConstraintSet {
    pub constraints: Vec<Constraint>,
}

impl ConstraintSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn top_to_top(mut self, target: Target, offset: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::Top,
            target,
            target_anchor: Anchor::Top,
            offset,
        });
        self
    }

    pub fn top_to_bottom(mut self, target: Target, offset: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::Top,
            target,
            target_anchor: Anchor::Bottom,
            offset,
        });
        self
    }

    pub fn bottom_to_bottom(mut self, target: Target, offset: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::Bottom,
            target,
            target_anchor: Anchor::Bottom,
            offset,
        });
        self
    }

    pub fn left_to_left(mut self, target: Target, offset: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::Left,
            target,
            target_anchor: Anchor::Left,
            offset,
        });
        self
    }

    pub fn left_to_right(mut self, target: Target, offset: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::Left,
            target,
            target_anchor: Anchor::Right,
            offset,
        });
        self
    }

    pub fn right_to_right(mut self, target: Target, offset: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::Right,
            target,
            target_anchor: Anchor::Right,
            offset,
        });
        self
    }

    pub fn right_to_left(mut self, target: Target, offset: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::Right,
            target,
            target_anchor: Anchor::Left,
            offset,
        });
        self
    }

    pub fn center_x(mut self, target: Target, offset: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::CenterX,
            target,
            target_anchor: Anchor::CenterX,
            offset,
        });
        self
    }

    pub fn center_y(mut self, target: Target, offset: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::CenterY,
            target,
            target_anchor: Anchor::CenterY,
            offset,
        });
        self
    }

    pub fn width(mut self, val: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::Width,
            target: Target::Parent,
            target_anchor: Anchor::Width,
            offset: val,
        });
        self
    }

    pub fn height(mut self, val: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::Height,
            target: Target::Parent,
            target_anchor: Anchor::Height,
            offset: val,
        });
        self
    }

    // Semantic helpers
    pub fn after(self, target: Target, offset: f32) -> Self {
        self.left_to_right(target, offset)
    }

    pub fn before(self, target: Target, offset: f32) -> Self {
        self.right_to_left(target, offset)
    }

    pub fn below(self, target: Target, offset: f32) -> Self {
        self.top_to_bottom(target, offset)
    }

    pub fn above(self, target: Target, offset: f32) -> Self {
        self.bottom_to_top(target, offset)
    }

    pub fn bottom_to_top(mut self, target: Target, offset: f32) -> Self {
        self.constraints.push(Constraint {
            anchor: Anchor::Bottom,
            target,
            target_anchor: Anchor::Top,
            offset,
        });
        self
    }
}

pub struct ConstraintSolver {
    pub parent_rect: Rect,
}

impl ConstraintSolver {
    pub fn new(parent_rect: Rect) -> Self {
        Self { parent_rect }
    }

    pub fn solve(&self, children_constraints: &[ConstraintSet], children_rects: &mut [Rect]) {
        for _ in 0..3 {
            // 3 passes should handle most simple chains
            for (i, set) in children_constraints.iter().enumerate() {
                let mut rect = children_rects[i];

                for c in &set.constraints {
                    let target_rect = match c.target {
                        Target::Parent => self.parent_rect,
                        Target::Child(idx) => children_rects[idx],
                    };

                    let target_val = match c.target_anchor {
                        Anchor::Top => target_rect.y,
                        Anchor::Bottom => target_rect.y + target_rect.height,
                        Anchor::Left => target_rect.x,
                        Anchor::Right => target_rect.x + target_rect.width,
                        Anchor::CenterX => target_rect.x + target_rect.width / 2.0,
                        Anchor::CenterY => target_rect.y + target_rect.height / 2.0,
                        Anchor::Width => target_rect.width,
                        Anchor::Height => target_rect.height,
                        Anchor::AspectRatio => target_rect.width / target_rect.height,
                    };

                    match c.anchor {
                        Anchor::Top => {
                            rect.y = target_val + c.offset;
                        }
                        Anchor::Bottom => {
                            let top_constrained =
                                set.constraints.iter().any(|cx| cx.anchor == Anchor::Top);
                            if top_constrained {
                                rect.height = (target_val + c.offset - rect.y).max(0.0);
                            } else {
                                rect.y = target_val + c.offset - rect.height;
                            }
                        }
                        Anchor::Left => {
                            rect.x = target_val + c.offset;
                        }
                        Anchor::Right => {
                            let left_constrained =
                                set.constraints.iter().any(|cx| cx.anchor == Anchor::Left);
                            if left_constrained {
                                rect.width = (target_val + c.offset - rect.x).max(0.0);
                            } else {
                                rect.x = target_val + c.offset - rect.width;
                            }
                        }
                        Anchor::CenterX => {
                            rect.x = target_val + c.offset - rect.width / 2.0;
                        }
                        Anchor::CenterY => {
                            rect.y = target_val + c.offset - rect.height / 2.0;
                        }
                        Anchor::Width => {
                            rect.width = c.offset;
                        }
                        Anchor::Height => {
                            rect.height = c.offset;
                        }
                        Anchor::AspectRatio => {
                            // c.offset is the ratio (width / height)
                            if rect.width > 0.0 && rect.height == 0.0 {
                                rect.height = rect.width / c.offset;
                            } else if rect.height > 0.0 && rect.width == 0.0 {
                                rect.width = rect.height * c.offset;
                            }
                        }
                    }
                }

                children_rects[i] = rect;
            }
        }
    }
}
