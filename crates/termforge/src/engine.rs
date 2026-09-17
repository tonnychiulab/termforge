use crate::audio::{AudioBackend, SoundEventQueue, TerminalBellBackend};
use crate::ecs::World;
use crate::input::{InputState, Key};
use crate::render::{Buffer2D, DoubleBuffer};
use crate::terminal::TerminalGuard;
use std::io::{self, stdout};
use std::thread;
use std::time::{Duration, Instant};

/// Configuration parameters for the TermForge engine instance.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub target_fps: u32,
    pub physics_hz: u32,
    pub auto_resize: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            target_fps: 60,
            physics_hz: 60,
            auto_resize: true,
        }
    }
}

/// Runtime Context exposed to games and scenes during update & render ticks.
pub struct GameContext {
    pub input: InputState,
    pub world: World,
    pub sound_queue: SoundEventQueue,
    pub width: u16,
    pub height: u16,
    pub should_quit: bool,
    pub frame_count: u64,
}

/// The user-facing Game trait implemented by arcade demos.
pub trait Game {
    fn init(&mut self, ctx: &mut GameContext);
    fn update(&mut self, ctx: &mut GameContext, dt: f32);
    fn render(&self, ctx: &GameContext, buffer: &mut Buffer2D);
}

/// The core TermForge Game Engine orchestrator.
pub struct Engine {
    config: EngineConfig,
    audio: Box<dyn AudioBackend>,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        Self {
            config,
            audio: Box::new(TerminalBellBackend::default()),
        }
    }

    pub fn with_audio_backend(mut self, backend: Box<dyn AudioBackend>) -> Self {
        self.audio = backend;
        self
    }

    /// Run the main game loop until quit is signaled.
    pub fn run<G: Game>(&mut self, mut game: G) -> io::Result<()> {
        let _guard = TerminalGuard::new()?;
        let (mut width, mut height) = TerminalGuard::size()?;

        let mut double_buffer = DoubleBuffer::new(width, height);
        let mut input = InputState::new();
        input.start_input_thread();

        let mut ctx = GameContext {
            input,
            world: World::new(),
            sound_queue: SoundEventQueue::new(),
            width,
            height,
            should_quit: false,
            frame_count: 0,
        };

        game.init(&mut ctx);

        let dt_fixed = Duration::from_micros((1_000_000 / self.config.physics_hz) as u64);
        let frame_budget = Duration::from_micros((1_000_000 / self.config.target_fps) as u64);
        let max_accumulator = Duration::from_millis(250);

        let mut last_tick = Instant::now();
        let mut accumulator = Duration::ZERO;
        let mut stdout_handle = stdout();

        while !ctx.should_quit {
            let now = Instant::now();
            let mut delta = now.duration_since(last_tick);
            last_tick = now;

            // Clamp delta to prevent spiral of death
            if delta > max_accumulator {
                delta = max_accumulator;
            }
            accumulator += delta;

            // Check terminal dimensions if resize enabled
            if self.config.auto_resize {
                if let Ok((new_w, new_h)) = TerminalGuard::size() {
                    if new_w != width || new_h != height {
                        width = new_w;
                        height = new_h;
                        ctx.width = width;
                        ctx.height = height;
                        double_buffer.resize(width, height);
                    }
                }
            }

            // Poll input events
            ctx.input.update();

            // Default engine quit shortcut: Ctrl+Q or ESC
            if ctx.input.is_pressed(Key::Ctrl('q')) {
                ctx.should_quit = true;
                break;
            }

            // Fixed-timestep physics/logic updates
            let dt_secs = dt_fixed.as_secs_f32();
            while accumulator >= dt_fixed {
                game.update(&mut ctx, dt_secs);
                accumulator -= dt_fixed;
            }

            // Flush sound events
            ctx.sound_queue.flush(&mut *self.audio);

            // Render cycle
            double_buffer.back.clear();
            game.render(&ctx, &mut double_buffer.back);
            let _ = double_buffer.flush(&mut stdout_handle);

            ctx.frame_count += 1;

            // Frame rate limiter
            let elapsed = now.elapsed();
            if elapsed < frame_budget {
                thread::sleep(frame_budget - elapsed);
            }
        }

        Ok(())
    }
}
