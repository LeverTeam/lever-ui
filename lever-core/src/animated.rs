use crate::animation::{Ease, Spring, SpringController};
use crate::state::{get_or_set_state, update_state};
use crate::types::Color;

pub trait Animatable: Clone + 'static {
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}

impl Animatable for f32 {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Animatable for Color {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        Color::lerp(*self, *other, t)
    }
}

pub struct Animated<T: Animatable> {
    _id: String,
    target: T,
    config: AnimationConfig,
}

pub enum AnimationConfig {
    Spring(Spring),

    Ease { duration: f32, ease: Ease },
}

impl<T: Animatable> Animated<T> {
    pub fn new(id: impl Into<String>, target: T) -> Self {
        Self {
            _id: id.into(),
            target,
            config: AnimationConfig::Spring(Spring::SNAPPY),
        }
    }

    pub fn with_spring(mut self, spring: Spring) -> Self {
        self.config = AnimationConfig::Spring(spring);
        self
    }

    pub fn with_ease(mut self, duration: f32, ease: Ease) -> Self {
        self.config = AnimationConfig::Ease { duration, ease };
        self
    }

    pub fn get(&self) -> T {
        self.target.clone()
    }
}

pub fn animated_spring(id: &str, target: f32, spring: Spring) -> f32 {
    let controller = get_or_set_state(id, || SpringController::new(target, spring));

    if (controller.target - target).abs() > 0.0001 {
        update_state(id, |c: &mut SpringController| {
            c.set_target(target);
        });
    }

    controller.value
}

pub fn animated_color(id: &str, target: Color, _duration: f32) -> Color {
    let r_id = format!("{}_r", id);
    let g_id = format!("{}_g", id);
    let b_id = format!("{}_b", id);
    let a_id = format!("{}_a", id);

    let r = animated_spring(&r_id, target.r, Spring::SNAPPY);
    let g = animated_spring(&g_id, target.g, Spring::SNAPPY);
    let b = animated_spring(&b_id, target.b, Spring::SNAPPY);
    let a = animated_spring(&a_id, target.a, Spring::SNAPPY);

    Color::rgba(r, g, b, a)
}
