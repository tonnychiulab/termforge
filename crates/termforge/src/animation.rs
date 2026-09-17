use crate::math::Vec2;
use crate::render::Color;
use std::f32::consts::PI;

/// Easing functions for smooth animations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EaseFunction {
    Linear,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    SineIn,
    SineOut,
    SineInOut,
}

impl EaseFunction {
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::QuadIn => t * t,
            Self::QuadOut => t * (2.0 - t),
            Self::QuadInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Self::CubicIn => t * t * t,
            Self::CubicOut => {
                let f = t - 1.0;
                f * f * f + 1.0
            }
            Self::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let f = 2.0 * t - 2.0;
                    0.5 * f * f * f + 1.0
                }
            }
            Self::SineIn => 1.0 - (t * PI * 0.5).cos(),
            Self::SineOut => (t * PI * 0.5).sin(),
            Self::SineInOut => 0.5 * (1.0 - (PI * t).cos()),
        }
    }
}

/// Tween interpolation container.
#[derive(Debug, Clone)]
pub struct Tween {
    pub start: f32,
    pub target: f32,
    pub duration: f32,
    pub elapsed: f32,
    pub ease: EaseFunction,
    pub completed: bool,
}

impl Tween {
    pub fn new(start: f32, target: f32, duration: f32, ease: EaseFunction) -> Self {
        Self {
            start,
            target,
            duration: duration.max(0.0001),
            elapsed: 0.0,
            ease,
            completed: false,
        }
    }

    pub fn update(&mut self, dt: f32) -> f32 {
        if self.completed {
            return self.target;
        }

        self.elapsed += dt;
        if self.elapsed >= self.duration {
            self.completed = true;
            self.target
        } else {
            let progress = self.elapsed / self.duration;
            let eased = self.ease.apply(progress);
            self.start + (self.target - self.start) * eased
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayMode {
    Loop,
    Once,
}

/// Frame-based ASCII character animation.
#[derive(Debug, Clone)]
pub struct SpriteAnimation {
    pub frames: Vec<char>,
    pub frame_duration: f32,
    pub elapsed: f32,
    pub current_frame: usize,
    pub play_mode: PlayMode,
    pub finished: bool,
}

impl SpriteAnimation {
    pub fn new(frames: Vec<char>, fps: f32, play_mode: PlayMode) -> Self {
        let frame_duration = 1.0 / fps.max(0.1);
        Self {
            frames,
            frame_duration,
            elapsed: 0.0,
            current_frame: 0,
            play_mode,
            finished: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.finished || self.frames.is_empty() {
            return;
        }

        self.elapsed += dt;
        while self.elapsed >= self.frame_duration {
            self.elapsed -= self.frame_duration;
            if self.current_frame + 1 < self.frames.len() {
                self.current_frame += 1;
            } else {
                match self.play_mode {
                    PlayMode::Loop => self.current_frame = 0,
                    PlayMode::Once => {
                        self.finished = true;
                        break;
                    }
                }
            }
        }
    }

    pub fn current_char(&self) -> char {
        self.frames.get(self.current_frame).copied().unwrap_or(' ')
    }
}

/// Particle instance.
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub ch: char,
    pub color: Color,
    pub active: bool,
}

/// High-performance Particle Emitter with pre-allocated memory pool.
pub struct ParticleEmitter {
    pool: Vec<Particle>,
}

impl ParticleEmitter {
    pub fn new(capacity: usize) -> Self {
        let default_particle = Particle {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            lifetime: 0.0,
            max_lifetime: 1.0,
            ch: '*',
            color: Color::White,
            active: false,
        };
        Self {
            pool: vec![default_particle; capacity],
        }
    }

    pub fn emit(&mut self, position: Vec2, velocity: Vec2, lifetime: f32, ch: char, color: Color) {
        if let Some(particle) = self.pool.iter_mut().find(|p| !p.active) {
            particle.position = position;
            particle.velocity = velocity;
            particle.lifetime = lifetime;
            particle.max_lifetime = lifetime;
            particle.ch = ch;
            particle.color = color;
            particle.active = true;
        }
    }

    pub fn burst(
        &mut self,
        center: Vec2,
        count: usize,
        speed_range: (f32, f32),
        lifetime: f32,
        chars: &[char],
        color: Color,
    ) {
        for i in 0..count {
            let angle = (i as f32 / count as f32) * PI * 2.0;
            let speed = speed_range.0 + (speed_range.1 - speed_range.0) * ((i % 3) as f32 / 2.0);
            let vel = Vec2::from_angle(angle) * speed;
            let ch = chars[i % chars.len()];
            self.emit(center, vel, lifetime, ch, color);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for p in self.pool.iter_mut().filter(|p| p.active) {
            p.position += p.velocity * dt;
            p.lifetime -= dt;
            if p.lifetime <= 0.0 {
                p.active = false;
            }
        }
    }

    pub fn active_particles(&self) -> impl Iterator<Item = &Particle> {
        self.pool.iter().filter(|p| p.active)
    }
}
