use std::f32::consts::PI;
use termforge::prelude::*;

const SHIP_RADIUS: f32 = 1.0;
const BULLET_RADIUS: f32 = 0.5;
const ASTEROID_LARGE_RADIUS: f32 = 3.0;
const ASTEROID_MEDIUM_RADIUS: f32 = 2.0;
const ASTEROID_SMALL_RADIUS: f32 = 1.0;
const EMP_BLAST_RADIUS: f32 = 22.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AsteroidSize {
    Large,
    Medium,
    Small,
}

impl AsteroidSize {
    fn radius(&self) -> f32 {
        match self {
            Self::Large => ASTEROID_LARGE_RADIUS,
            Self::Medium => ASTEROID_MEDIUM_RADIUS,
            Self::Small => ASTEROID_SMALL_RADIUS,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Large => 20,
            Self::Medium => 50,
            Self::Small => 100,
        }
    }

    fn glyph(&self) -> char {
        match self {
            Self::Large => '⬤',
            Self::Medium => '◉',
            Self::Small => '•',
        }
    }
}

struct Ship {
    pos: Vec2,
    vel: Vec2,
    angle: f32,
    fire_cooldown: f32,
    invincible_timer: f32,
    alive: bool,
    bombs: u32,
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    lifetime: f32,
}

struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    size: AsteroidSize,
}

struct AsteroidsGame {
    ship: Ship,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    particles: ParticleEmitter,
    score: u32,
    lives: u32,
    wave: u32,
    game_over: bool,
    paused: bool,
    sound_muted: bool,
    respawn_timer: f32,
    seed: u32,
}

impl Default for AsteroidsGame {
    fn default() -> Self {
        Self {
            ship: Ship {
                pos: Vec2::new(40.0, 15.0),
                vel: Vec2::ZERO,
                angle: -PI * 0.5,
                fire_cooldown: 0.0,
                invincible_timer: 3.0,
                alive: true,
                bombs: 1,
            },
            bullets: Vec::new(),
            asteroids: Vec::new(),
            particles: ParticleEmitter::new(400),
            score: 0,
            lives: 3,
            wave: 1,
            game_over: false,
            paused: false,
            sound_muted: false,
            respawn_timer: 0.0,
            seed: 42,
        }
    }
}

impl AsteroidsGame {
    fn pseudo_random(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.seed as f32) / (u32::MAX as f32)
    }

    fn spawn_wave(&mut self, width: f32, height: f32) {
        self.asteroids.clear();
        let count = 3 + self.wave;
        for i in 0..count {
            let angle = (i as f32 / count as f32) * PI * 2.0;
            let pos = Vec2::new(width * 0.5, height * 0.5) + Vec2::from_angle(angle) * (height * 0.4);
            let vel = Vec2::from_angle(angle + 1.2) * (4.0 + self.wave as f32 * 0.5);
            self.asteroids.push(Asteroid {
                pos,
                vel,
                size: AsteroidSize::Large,
            });
        }
    }

    fn wrap(pos: &mut Vec2, width: f32, height: f32) {
        if pos.x < 0.0 {
            pos.x += width;
        } else if pos.x >= width {
            pos.x -= width;
        }
        if pos.y < 0.0 {
            pos.y += height;
        } else if pos.y >= height {
            pos.y -= height;
        }
    }

    fn hyperspace(&mut self, width: f32, height: f32, sound_queue: &mut SoundEventQueue) {
        self.particles.burst(self.ship.pos, 25, (4.0, 12.0), 0.6, &['✦', '·'], Color::BrightCyan);
        self.ship.pos = Vec2::new(
            5.0 + self.pseudo_random() * (width - 10.0),
            5.0 + self.pseudo_random() * (height - 10.0),
        );
        self.ship.vel = Vec2::ZERO;
        self.ship.invincible_timer = 1.5;
        self.particles.burst(self.ship.pos, 20, (3.0, 8.0), 0.5, &['★', '✦'], Color::BrightGreen);
        sound_queue.trigger("warp");
    }

    fn emp_blast(&mut self, sound_queue: &mut SoundEventQueue) {
        self.ship.bombs -= 1;
        sound_queue.trigger("emp");
        self.particles.burst(self.ship.pos, 60, (8.0, 20.0), 0.8, &['#', '✦', '★', '·'], Color::BrightCyan);

        let mut destroyed_asteroids = Vec::new();
        let mut new_asteroids = Vec::new();

        for (a_idx, asteroid) in self.asteroids.iter().enumerate() {
            if self.ship.pos.distance(asteroid.pos) <= EMP_BLAST_RADIUS {
                destroyed_asteroids.push(a_idx);
                self.score += asteroid.size.score();

                match asteroid.size {
                    AsteroidSize::Large => {
                        for k in [-1.0, 1.0] {
                            new_asteroids.push(Asteroid {
                                pos: asteroid.pos,
                                vel: asteroid.vel.rotate(k * 1.2) * 1.5,
                                size: AsteroidSize::Medium,
                            });
                        }
                    }
                    AsteroidSize::Medium => {
                        for k in [-1.0, 1.0] {
                            new_asteroids.push(Asteroid {
                                pos: asteroid.pos,
                                vel: asteroid.vel.rotate(k * 1.5) * 1.8,
                                size: AsteroidSize::Small,
                            });
                        }
                    }
                    AsteroidSize::Small => {}
                }
            }
        }

        destroyed_asteroids.sort_unstable();
        destroyed_asteroids.dedup();
        for &idx in destroyed_asteroids.iter().rev() {
            if idx < self.asteroids.len() {
                self.asteroids.remove(idx);
            }
        }
        self.asteroids.extend(new_asteroids);
    }
}

impl Game for AsteroidsGame {
    fn init(&mut self, ctx: &mut GameContext) {
        self.ship.pos = Vec2::new(ctx.width as f32 * 0.5, ctx.height as f32 * 0.5);
        self.spawn_wave(ctx.width as f32, ctx.height as f32);
    }

    fn update(&mut self, ctx: &mut GameContext, dt: f32) {
        let width = ctx.width as f32;
        let height = ctx.height as f32;

        // Pause Toggle
        if ctx.input.just_pressed(Key::Char('p')) || ctx.input.just_pressed(Key::Char('P')) {
            self.paused = !self.paused;
        }

        // Mute Toggle
        if ctx.input.just_pressed(Key::Char('m')) || ctx.input.just_pressed(Key::Char('M')) {
            self.sound_muted = !self.sound_muted;
        }

        if self.paused {
            return;
        }

        if self.game_over {
            if ctx.input.just_pressed(Key::Char('\n')) || ctx.input.just_pressed(Key::Char('r')) {
                *self = AsteroidsGame::default();
                self.init(ctx);
            }
            if ctx.input.just_pressed(Key::Esc) {
                ctx.should_quit = true;
            }
            return;
        }

        // Handle respawn timer
        if !self.ship.alive {
            self.respawn_timer -= dt;
            if self.respawn_timer <= 0.0 && self.lives > 0 {
                self.ship.alive = true;
                self.ship.bombs = 1;
                self.ship.pos = Vec2::new(width * 0.5, height * 0.5);
                self.ship.vel = Vec2::ZERO;
                self.ship.invincible_timer = 3.0;
            }
        }

        // Ship Controls
        if self.ship.alive {
            let rot_speed = 3.8;
            if ctx.input.is_pressed(Key::Left) || ctx.input.is_pressed(Key::Char('a')) {
                self.ship.angle -= rot_speed * dt;
            }
            if ctx.input.is_pressed(Key::Right) || ctx.input.is_pressed(Key::Char('d')) {
                self.ship.angle += rot_speed * dt;
            }

            // Forward Thrust (W / Up)
            if ctx.input.is_pressed(Key::Up) || ctx.input.is_pressed(Key::Char('w')) {
                let thrust = Vec2::from_angle(self.ship.angle) * 22.0;
                self.ship.vel += thrust * dt;
                self.ship.vel = self.ship.vel.clamp_length(18.0);

                let tail_pos = self.ship.pos - Vec2::from_angle(self.ship.angle) * 1.5;
                let particle_vel = -Vec2::from_angle(self.ship.angle) * 5.0 + Vec2::new(0.0, 0.5);
                self.particles.emit(tail_pos, particle_vel, 0.25, '·', Color::BrightYellow);
            }

            // Active Brake (S / Down)
            if ctx.input.is_pressed(Key::Down) || ctx.input.is_pressed(Key::Char('s')) {
                self.ship.vel *= (0.88_f32).powf(dt * 60.0);
                let nose = self.ship.pos + Vec2::from_angle(self.ship.angle) * 1.0;
                self.particles.emit(nose, Vec2::ZERO, 0.15, 'x', Color::BrightRed);
            }

            // Lateral Strafing (Q - Port, E - Starboard)
            if ctx.input.is_pressed(Key::Char('q')) {
                let left_normal = Vec2::new(-self.ship.angle.sin(), self.ship.angle.cos());
                self.ship.vel += left_normal * 18.0 * dt;
                self.ship.vel = self.ship.vel.clamp_length(18.0);
            }
            if ctx.input.is_pressed(Key::Char('e')) {
                let right_normal = Vec2::new(self.ship.angle.sin(), -self.ship.angle.cos());
                self.ship.vel += right_normal * 18.0 * dt;
                self.ship.vel = self.ship.vel.clamp_length(18.0);
            }

            // 180° Flip (X)
            if ctx.input.just_pressed(Key::Char('x')) || ctx.input.just_pressed(Key::Char('X')) {
                self.ship.angle += PI;
                self.particles.burst(self.ship.pos, 8, (2.0, 6.0), 0.2, &['·'], Color::BrightCyan);
            }

            // Hyperspace Jump (Z / H)
            if ctx.input.just_pressed(Key::Char('z')) || ctx.input.just_pressed(Key::Char('h')) {
                self.hyperspace(width, height, &mut ctx.sound_queue);
            }

            // EMP Smart Bomb (B / F)
            if (ctx.input.just_pressed(Key::Char('b')) || ctx.input.just_pressed(Key::Char('f'))) && self.ship.bombs > 0 {
                self.emp_blast(&mut ctx.sound_queue);
            }

            // Drag
            self.ship.vel *= (0.988_f32).powf(dt * 60.0);
            self.ship.pos += self.ship.vel * dt;
            Self::wrap(&mut self.ship.pos, width, height);

            // Cooldowns
            self.ship.fire_cooldown -= dt;
            self.ship.invincible_timer -= dt;

            // Firing
            if (ctx.input.is_pressed(Key::Char(' ')) || ctx.input.just_pressed(Key::Char(' ')))
                && self.ship.fire_cooldown <= 0.0
            {
                self.ship.fire_cooldown = 0.15;
                let bullet_vel = Vec2::from_angle(self.ship.angle) * 36.0;
                self.bullets.push(Bullet {
                    pos: self.ship.pos + Vec2::from_angle(self.ship.angle) * 1.2,
                    vel: bullet_vel,
                    lifetime: 1.2,
                });
                if !self.sound_muted {
                    ctx.sound_queue.trigger("fire");
                }
            }
        }

        // Update Bullets
        for b in &mut self.bullets {
            b.pos += b.vel * dt;
            b.lifetime -= dt;
            Self::wrap(&mut b.pos, width, height);
        }
        self.bullets.retain(|b| b.lifetime > 0.0);

        // Update Asteroids
        for a in &mut self.asteroids {
            a.pos += a.vel * dt;
            Self::wrap(&mut a.pos, width, height);
        }

        // Update Particles
        self.particles.update(dt);

        // Collision: Bullets vs Asteroids
        let mut new_asteroids = Vec::new();
        let mut destroyed_bullets = Vec::new();
        let mut destroyed_asteroids = Vec::new();

        for (b_idx, bullet) in self.bullets.iter().enumerate() {
            let b_circle = Circle::new(BULLET_RADIUS);
            for (a_idx, asteroid) in self.asteroids.iter().enumerate() {
                if destroyed_asteroids.contains(&a_idx) {
                    continue;
                }
                let a_circle = Circle::new(asteroid.size.radius());
                if b_circle.intersects(bullet.pos, &a_circle, asteroid.pos) {
                    destroyed_bullets.push(b_idx);
                    destroyed_asteroids.push(a_idx);
                    self.score += asteroid.size.score();

                    let p_count = match asteroid.size {
                        AsteroidSize::Large => 20,
                        AsteroidSize::Medium => 14,
                        AsteroidSize::Small => 8,
                    };
                    self.particles.burst(
                        asteroid.pos,
                        p_count,
                        (3.0, 10.0),
                        0.5,
                        &['*', '✦', '·'],
                        Color::BrightRed,
                    );

                    match asteroid.size {
                        AsteroidSize::Large => {
                            for k in [-1.0, 1.0] {
                                let vel = asteroid.vel.rotate(k * 0.8) * 1.3;
                                new_asteroids.push(Asteroid {
                                    pos: asteroid.pos,
                                    vel,
                                    size: AsteroidSize::Medium,
                                });
                            }
                        }
                        AsteroidSize::Medium => {
                            for k in [-1.0, 1.0] {
                                let vel = asteroid.vel.rotate(k * 1.0) * 1.6;
                                new_asteroids.push(Asteroid {
                                    pos: asteroid.pos,
                                    vel,
                                    size: AsteroidSize::Small,
                                });
                            }
                        }
                        AsteroidSize::Small => {}
                    }
                    break;
                }
            }
        }

        if !destroyed_bullets.is_empty() && !self.sound_muted {
            ctx.sound_queue.trigger("hit");
        }

        let mut b_indices = destroyed_bullets;
        b_indices.sort_unstable();
        b_indices.dedup();
        for &idx in b_indices.iter().rev() {
            if idx < self.bullets.len() {
                self.bullets.remove(idx);
            }
        }

        let mut a_indices = destroyed_asteroids;
        a_indices.sort_unstable();
        a_indices.dedup();
        for &idx in a_indices.iter().rev() {
            if idx < self.asteroids.len() {
                self.asteroids.remove(idx);
            }
        }

        self.asteroids.extend(new_asteroids);

        // Next Wave check
        if self.asteroids.is_empty() {
            self.wave += 1;
            self.spawn_wave(width, height);
            self.particles.burst(
                Vec2::new(width * 0.5, height * 0.5),
                40,
                (5.0, 15.0),
                1.0,
                &['★', '✦', '·'],
                Color::BrightGreen,
            );
        }

        // Collision: Ship vs Asteroids
        if self.ship.alive && self.ship.invincible_timer <= 0.0 {
            let ship_circle = Circle::new(SHIP_RADIUS);
            for asteroid in &self.asteroids {
                let a_circle = Circle::new(asteroid.size.radius());
                if ship_circle.intersects(self.ship.pos, &a_circle, asteroid.pos) {
                    self.ship.alive = false;
                    self.respawn_timer = 2.0;
                    self.lives = self.lives.saturating_sub(1);

                    self.particles.burst(
                        self.ship.pos,
                        40,
                        (4.0, 16.0),
                        0.8,
                        &['#', '%', '*', '✦'],
                        Color::BrightYellow,
                    );
                    if !self.sound_muted {
                        ctx.sound_queue.trigger("explosion");
                    }

                    if self.lives == 0 {
                        self.game_over = true;
                    }
                    break;
                }
            }
        }
    }

    fn render(&self, ctx: &GameContext, buffer: &mut Buffer2D) {
        // Draw Particles
        for p in self.particles.active_particles() {
            let px = p.position.x as u16;
            let py = p.position.y as u16;
            buffer.set(px, py, Cell::new(p.ch).with_fg(p.color));
        }

        // Draw Asteroids
        for a in &self.asteroids {
            let ax = a.pos.x as u16;
            let ay = a.pos.y as u16;
            let color = match a.size {
                AsteroidSize::Large => Color::BrightBlack,
                AsteroidSize::Medium => Color::White,
                AsteroidSize::Small => Color::Yellow,
            };
            buffer.set(ax, ay, Cell::new(a.size.glyph()).with_fg(color));
        }

        // Draw Bullets
        for b in &self.bullets {
            let bx = b.pos.x as u16;
            let by = b.pos.y as u16;
            buffer.set(bx, by, Cell::new('•').with_fg(Color::BrightCyan));
        }

        // Draw Ship
        if self.ship.alive {
            let blink = self.ship.invincible_timer > 0.0
                && ((self.ship.invincible_timer * 8.0) as u32 % 2 == 0);
            if !blink {
                let sx = self.ship.pos.x as u16;
                let sy = self.ship.pos.y as u16;
                let glyph = match ((self.ship.angle + PI * 0.125).rem_euclid(PI * 2.0) / (PI * 0.25)) as u32 {
                    0 => '▶',
                    1 => '◢',
                    2 => '▼',
                    3 => '◣',
                    4 => '◀',
                    5 => '◤',
                    6 => '▲',
                    _ => '◥',
                };
                buffer.set(sx, sy, Cell::new(glyph).with_fg(Color::BrightGreen).with_bold());
            }
        }

        // Enhanced HUD
        let score_str = format!(" SCORE: {} ", self.score);
        let wave_str = format!(" W:{} ", self.wave);
        let mut lives_str = String::from(" ");
        for _ in 0..self.lives {
            lives_str.push('♥');
        }
        let bomb_str = if self.ship.bombs > 0 { " EMP:[B] " } else { " EMP:-- " };
        let snd_str = if self.sound_muted { " [M]OFF " } else { " [M]ON " };

        buffer.draw_text(1, 0, &score_str, Color::BrightYellow, Color::Reset);
        buffer.draw_text(ctx.width / 2 - 9, 0, &lives_str, Color::BrightRed, Color::Reset);
        buffer.draw_text(ctx.width / 2 - 1, 0, bomb_str, Color::BrightCyan, Color::Reset);
        buffer.draw_text(ctx.width / 2 + 8, 0, snd_str, Color::BrightBlack, Color::Reset);
        buffer.draw_text(ctx.width.saturating_sub(wave_str.len() as u16 + 2), 0, &wave_str, Color::BrightCyan, Color::Reset);

        // Pause Modal
        if self.paused {
            let center_x = ctx.width / 2;
            let center_y = ctx.height / 2;
            let banner = "=== PAUSED ===";
            let sub = "Press P to Resume";
            buffer.draw_rect(center_x.saturating_sub(18), center_y.saturating_sub(2), 36, 5, Color::BrightYellow);
            buffer.draw_text(center_x.saturating_sub((banner.len() / 2) as u16), center_y.saturating_sub(1), banner, Color::BrightYellow, Color::Reset);
            buffer.draw_text(center_x.saturating_sub((sub.len() / 2) as u16), center_y + 1, sub, Color::White, Color::Reset);
        }

        // Draw Game Over Screen
        if self.game_over {
            let center_x = ctx.width / 2;
            let center_y = ctx.height / 2;
            let banner = "=== GAME OVER ===";
            let sub = "Press ENTER / R to Restart or Ctrl+Q to Quit";
            buffer.draw_rect(center_x.saturating_sub(25), center_y.saturating_sub(3), 50, 7, Color::BrightRed);
            buffer.draw_text(center_x.saturating_sub((banner.len() / 2) as u16), center_y.saturating_sub(1), banner, Color::BrightRed, Color::Reset);
            buffer.draw_text(center_x.saturating_sub((sub.len() / 2) as u16), center_y + 1, sub, Color::White, Color::Reset);
        }
    }
}

fn main() -> std::io::Result<()> {
    let config = EngineConfig {
        target_fps: 60,
        physics_hz: 60,
        auto_resize: true,
    };

    let mut engine = Engine::new(config);
    let game = AsteroidsGame::default();
    engine.run(game)
}
