use crate::types::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ease {
    Linear,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
}

impl Ease {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Ease::Linear => t,
            Ease::QuadIn => t * t,
            Ease::QuadOut => t * (2.0 - t),
            Ease::QuadInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Ease::CubicIn => t * t * t,
            Ease::CubicOut => {
                let t = t - 1.0;
                t * t * t + 1.0
            }
            Ease::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t = t - 1.0;
                    4.0 * t * t * t + 1.0
                }
            }
        }
    }
}

pub struct Animation {
    duration: f32,
    elapsed: f32,
    ease: Ease,
    active: bool,
    looping: bool,
}

impl Animation {
    pub fn new(duration: f32, ease: Ease) -> Self {
        Self {
            duration,
            elapsed: 0.0,
            ease,
            active: false,
            looping: false,
        }
    }

    pub fn with_loop(mut self) -> Self {
        self.looping = true;
        self
    }

    pub fn start(&mut self) {
        self.elapsed = 0.0;
        self.active = true;
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.active = false;
    }

    pub fn update(&mut self, dt: f32) {
        if self.active {
            self.elapsed += dt;
            if self.elapsed >= self.duration {
                if self.looping {
                    self.elapsed -= self.duration;
                } else {
                    self.elapsed = self.duration;
                    self.active = false;
                }
            }
        }
    }

    pub fn progress(&self) -> f32 {
        if self.duration <= 0.0 {
            return 1.0;
        }
        self.ease.apply((self.elapsed / self.duration).min(1.0))
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn value(&self, start: f32, end: f32) -> f32 {
        start + (end - start) * self.progress()
    }

    pub fn color(&self, start: Color, end: Color) -> Color {
        let p = self.progress();
        Color {
            r: start.r + (end.r - start.r) * p,
            g: start.g + (end.g - start.g) * p,
            b: start.b + (end.b - start.b) * p,
            a: start.a + (end.a - start.a) * p,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimationController {
    value: f32,
    start_value: f32,
    target: f32,
    duration: f32,
    ease: Ease,
    elapsed: f32,
    is_animating: bool,
}

impl AnimationController {
    pub fn new(initial: f32) -> Self {
        Self {
            value: initial,
            start_value: initial,
            target: initial,
            duration: 0.3,
            ease: Ease::QuadOut,
            elapsed: 0.0,
            is_animating: false,
        }
    }

    pub fn animate_to(&mut self, target: f32, duration: f32, ease: Ease) {
        if (target - self.target).abs() < 0.001 && !self.is_animating {
            return;
        }
        self.start_value = self.value;
        self.target = target;
        self.duration = duration;
        self.ease = ease;
        self.elapsed = 0.0;
        self.is_animating = true;
    }

    pub fn tick(&mut self, dt: f32) {
        if !self.is_animating {
            return;
        }

        self.elapsed += dt;
        let t = (self.elapsed / self.duration).min(1.0);
        let eased_t = self.ease.apply(t);

        self.value = self.start_value + (self.target - self.start_value) * eased_t;

        if t >= 1.0 {
            self.value = self.target;
            self.is_animating = false;
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }
}
