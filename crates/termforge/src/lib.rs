//! # TermForge
//! 
//! A high-performance, double-buffered terminal game engine written in pure Rust.
//! Designed for real-time, interactive 60 FPS simulations with an ECS architecture,
//! spatial indexing, and particle/tween animation systems.

pub mod animation;
pub mod collision;
pub mod ecs;
pub mod math;
pub mod render;

#[cfg(not(target_arch = "wasm32"))]
pub mod audio;
#[cfg(not(target_arch = "wasm32"))]
pub mod engine;
#[cfg(not(target_arch = "wasm32"))]
pub mod input;
#[cfg(not(target_arch = "wasm32"))]
pub mod scene;
#[cfg(not(target_arch = "wasm32"))]
pub mod terminal;

pub mod prelude {
    pub use crate::animation::{EaseFunction, Particle, ParticleEmitter, PlayMode, SpriteAnimation, Tween};
    pub use crate::collision::{AABB, Circle, CollisionLayer, CollisionResult, Quadtree};
    pub use crate::ecs::{ComponentPool, EntityId, GenerationalArena, World};
    pub use crate::math::{Rect, Transform, Vec2};
    pub use crate::render::{Buffer2D, Cell, Color, DoubleBuffer, StyleModifier};

    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::audio::{AudioBackend, SoundEventQueue, TerminalBellBackend};
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::engine::{Engine, EngineConfig, Game, GameContext};
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::input::{InputEvent, InputState, Key};
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::scene::{Camera2D, Scene, SceneManager, Transition};
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::terminal::TerminalGuard;
}
