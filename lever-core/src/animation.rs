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

/// Parameters for a spring animation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
}

impl Spring {
    /// A snappy, highly responsive spring with minimal overshoot.
    pub const SNAPPY: Self = Self {
        stiffness: 230.0,
        damping: 22.0,
        mass: 1.0,
    };

    /// A smooth, gentle spring with no overshoot (critically damped).
    pub const SMOOTH: Self = Self {
        stiffness: 120.0,
        damping: 22.0,
        mass: 1.0,
    };

    /// A bouncy, playful spring with noticeable oscillation.
    pub const BOUNCY: Self = Self {
        stiffness: 180.0,
        damping: 12.0,
        mass: 1.0,
    };

    /// An over-damped, slow spring.
    pub const GENTLE: Self = Self {
        stiffness: 50.0,
        damping: 10.0,
        mass: 1.0,
    };
}

impl Default for Spring {
    fn default() -> Self {
        Self::SNAPPY
    }
}

#[derive(Debug, Clone)]
pub struct SpringController {
    pub value: f32,
    pub velocity: f32,
    pub target: f32,
    pub spring: Spring,
    pub precision: f32,
}

impl SpringController {
    pub fn new(initial: f32, spring: Spring) -> Self {
        Self {
            value: initial,
            velocity: 0.0,
            target: initial,
            spring,
            precision: 0.001,
        }
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn tick(&mut self, dt: f32) {
        // Use a fixed time step internally for stability (120Hz)
        let sub_step = 1.0 / 120.0;
        let mut remaining_dt = dt.min(0.1); // Cap dt to avoid explosion

        while remaining_dt > 0.0 {
            let step = remaining_dt.min(sub_step);

            let spring_force = -self.spring.stiffness * (self.value - self.target);
            let damping_force = -self.spring.damping * self.velocity;
            let acceleration = (spring_force + damping_force) / self.spring.mass;

            self.velocity += acceleration * step;
            self.value += self.velocity * step;

            remaining_dt -= step;
        }

        // Snap to target if very close and slow
        if (self.value - self.target).abs() < self.precision && self.velocity.abs() < self.precision
        {
            self.value = self.target;
            self.velocity = 0.0;
        }
    }

    pub fn is_finished(&self) -> bool {
        (self.value - self.target).abs() < self.precision && self.velocity.abs() < self.precision
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

    pub fn set_target(&mut self, target: f32) {
        self.animate_to(target, 0.2, Ease::QuadInOut);
    }

    pub fn update(&mut self, dt: f32, duration: f32) {
        if (self.target - self.value).abs() > 0.001 {
            self.animate_to(self.target, duration, Ease::QuadInOut);
        }
        self.tick(dt);
    }

    pub fn reset(&mut self, value: f32) {
        self.value = value;
        self.start_value = value;
        self.target = value;
        self.elapsed = 0.0;
        self.is_animating = false;
    }

    pub fn is_finished(&self) -> bool {
        !self.is_animating
    }
}
