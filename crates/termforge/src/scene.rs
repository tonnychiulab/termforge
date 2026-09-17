use crate::engine::GameContext;
use crate::math::Vec2;
use crate::render::Buffer2D;

pub trait Scene {
    fn on_enter(&mut self, _ctx: &mut GameContext) {}
    fn on_exit(&mut self, _ctx: &mut GameContext) {}
    fn update(&mut self, ctx: &mut GameContext, dt: f32);
    fn render(&self, ctx: &GameContext, buffer: &mut Buffer2D);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transition {
    None,
    Fade(f32),
}

/// Stack-based Scene Manager.
pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    pub fn new() -> Self {
        Self { scenes: Vec::new() }
    }

    pub fn push(&mut self, mut scene: Box<dyn Scene>, ctx: &mut GameContext) {
        scene.on_enter(ctx);
        self.scenes.push(scene);
    }

    pub fn pop(&mut self, ctx: &mut GameContext) -> Option<Box<dyn Scene>> {
        if let Some(mut scene) = self.scenes.pop() {
            scene.on_exit(ctx);
            Some(scene)
        } else {
            None
        }
    }

    pub fn current_mut(&mut self) -> Option<&mut Box<dyn Scene>> {
        self.scenes.last_mut()
    }

    pub fn current(&self) -> Option<&Box<dyn Scene>> {
        self.scenes.last()
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.is_empty()
    }
}

/// 2D Camera supporting smooth following and viewport boundaries.
pub struct Camera2D {
    pub position: Vec2,
    pub target: Vec2,
    pub smoothing: f32,
    pub viewport_size: Vec2,
}

impl Camera2D {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            target: Vec2::ZERO,
            smoothing: 0.1,
            viewport_size: Vec2::new(viewport_width, viewport_height),
        }
    }

    pub fn update(&mut self) {
        self.position = self.position.lerp(self.target, self.smoothing);
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        world_pos - self.position + self.viewport_size * 0.5
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        screen_pos + self.position - self.viewport_size * 0.5
    }
}
