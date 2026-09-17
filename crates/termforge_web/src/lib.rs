use std::collections::HashSet;
use std::f32::consts::PI;
use termforge::prelude::*;
use wasm_bindgen::prelude::*;

const SHIP_RADIUS: f32 = 1.0;
const BULLET_RADIUS: f32 = 0.5;
const ASTEROID_LARGE_RADIUS: f32 = 3.0;
const ASTEROID_MEDIUM_RADIUS: f32 = 2.0;
const ASTEROID_SMALL_RADIUS: f32 = 1.0;

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

#[wasm_bindgen]
pub struct WasmAsteroidsGame {
    ship: Ship,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    particles: ParticleEmitter,
    score: u32,
    lives: u32,
    wave: u32,
    game_over: bool,
    respawn_timer: f32,
    width: u16,
    height: u16,
    double_buffer: DoubleBuffer,
    pressed_keys: HashSet<String>,
    sound_effect: Option<String>,
}

#[wasm_bindgen]
impl WasmAsteroidsGame {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u16, height: u16) -> Self {
        let mut game = Self {
            ship: Ship {
                pos: Vec2::new(width as f32 * 0.5, height as f32 * 0.5),
                vel: Vec2::ZERO,
                angle: -PI * 0.5,
                fire_cooldown: 0.0,
                invincible_timer: 3.0,
                alive: true,
            },
            bullets: Vec::new(),
            asteroids: Vec::new(),
            particles: ParticleEmitter::new(300),
            score: 0,
            lives: 3,
            wave: 1,
            game_over: false,
            respawn_timer: 0.0,
            width,
            height,
            double_buffer: DoubleBuffer::new(width, height),
            pressed_keys: HashSet::new(),
            sound_effect: None,
        };
        game.spawn_wave();
        game
    }

    pub fn key_down(&mut self, key: &str) {
        self.pressed_keys.insert(key.to_lowercase());
        if self.game_over && (key == "Enter" || key == "r" || key == "R") {
            self.reset();
        }
    }

    pub fn key_up(&mut self, key: &str) {
        self.pressed_keys.remove(&key.to_lowercase());
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.lives = 3;
        self.wave = 1;
        self.game_over = false;
        self.respawn_timer = 0.0;
        self.bullets.clear();
        self.ship.alive = true;
        self.ship.pos = Vec2::new(self.width as f32 * 0.5, self.height as f32 * 0.5);
        self.ship.vel = Vec2::ZERO;
        self.ship.angle = -PI * 0.5;
        self.ship.invincible_timer = 3.0;
        self.spawn_wave();
    }

    fn spawn_wave(&mut self) {
        self.asteroids.clear();
        let count = 3 + self.wave;
        let w = self.width as f32;
        let h = self.height as f32;
        for i in 0..count {
            let angle = (i as f32 / count as f32) * PI * 2.0;
            let pos = Vec2::new(w * 0.5, h * 0.5) + Vec2::from_angle(angle) * (h * 0.4);
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

    /// Advance game state by dt and return the ANSI diff string to be passed into xterm.js
    pub fn tick(&mut self, dt: f32) -> String {
        let w = self.width as f32;
        let h = self.height as f32;
        self.sound_effect = None;

        if !self.game_over {
            // Respawn handling
            if !self.ship.alive {
                self.respawn_timer -= dt;
                if self.respawn_timer <= 0.0 && self.lives > 0 {
                    self.ship.alive = true;
                    self.ship.pos = Vec2::new(w * 0.5, h * 0.5);
                    self.ship.vel = Vec2::ZERO;
                    self.ship.invincible_timer = 3.0;
                }
            }

            // Ship inputs
            if self.ship.alive {
                let rot_speed = 3.5;
                if self.pressed_keys.contains("arrowleft") || self.pressed_keys.contains("a") {
                    self.ship.angle -= rot_speed * dt;
                }
                if self.pressed_keys.contains("arrowright") || self.pressed_keys.contains("d") {
                    self.ship.angle += rot_speed * dt;
                }
                if self.pressed_keys.contains("arrowup") || self.pressed_keys.contains("w") {
                    let thrust = Vec2::from_angle(self.ship.angle) * 20.0;
                    self.ship.vel += thrust * dt;
                    self.ship.vel = self.ship.vel.clamp_length(18.0);

                    let tail = self.ship.pos - Vec2::from_angle(self.ship.angle) * 1.5;
                    let p_vel = -Vec2::from_angle(self.ship.angle) * 5.0;
                    self.particles.emit(tail, p_vel, 0.25, '·', Color::BrightYellow);
                }

                self.ship.vel *= (0.985_f32).powf(dt * 60.0);
                self.ship.pos += self.ship.vel * dt;
                Self::wrap(&mut self.ship.pos, w, h);

                self.ship.fire_cooldown -= dt;
                self.ship.invincible_timer -= dt;

                if self.pressed_keys.contains(" ") && self.ship.fire_cooldown <= 0.0 {
                    self.ship.fire_cooldown = 0.15;
                    let bullet_vel = Vec2::from_angle(self.ship.angle) * 35.0;
                    self.bullets.push(Bullet {
                        pos: self.ship.pos + Vec2::from_angle(self.ship.angle) * 1.2,
                        vel: bullet_vel,
                        lifetime: 1.2,
                    });
                    self.sound_effect = Some("fire".into());
                }
            }

            // Bullets
            for b in &mut self.bullets {
                b.pos += b.vel * dt;
                b.lifetime -= dt;
                Self::wrap(&mut b.pos, w, h);
            }
            self.bullets.retain(|b| b.lifetime > 0.0);

            // Asteroids
            for a in &mut self.asteroids {
                a.pos += a.vel * dt;
                Self::wrap(&mut a.pos, w, h);
            }

            // Particles
            self.particles.update(dt);

            // Collisions: Bullets vs Asteroids
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

                        self.particles.burst(
                            asteroid.pos,
                            18,
                            (3.0, 10.0),
                            0.5,
                            &['*', '✦', '·'],
                            Color::BrightRed,
                        );
                        self.sound_effect = Some("hit".into());

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

            if self.asteroids.is_empty() {
                self.wave += 1;
                self.spawn_wave();
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
                            35,
                            (4.0, 15.0),
                            0.8,
                            &['#', '%', '*', '✦'],
                            Color::BrightYellow,
                        );
                        self.sound_effect = Some("explosion".into());

                        if self.lives == 0 {
                            self.game_over = true;
                        }
                        break;
                    }
                }
            }
        }

        // Render to back buffer
        self.double_buffer.back.clear();

        // Particles
        for p in self.particles.active_particles() {
            let px = p.position.x as u16;
            let py = p.position.y as u16;
            self.double_buffer.back.set(px, py, Cell::new(p.ch).with_fg(p.color));
        }

        // Asteroids
        for a in &self.asteroids {
            let ax = a.pos.x as u16;
            let ay = a.pos.y as u16;
            let color = match a.size {
                AsteroidSize::Large => Color::BrightBlack,
                AsteroidSize::Medium => Color::White,
                AsteroidSize::Small => Color::Yellow,
            };
            self.double_buffer.back.set(ax, ay, Cell::new(a.size.glyph()).with_fg(color));
        }

        // Bullets
        for b in &self.bullets {
            let bx = b.pos.x as u16;
            let by = b.pos.y as u16;
            self.double_buffer.back.set(bx, by, Cell::new('•').with_fg(Color::BrightCyan));
        }

        // Ship
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
                self.double_buffer.back.set(sx, sy, Cell::new(glyph).with_fg(Color::BrightGreen).with_bold());
            }
        }

        // HUD
        let score_str = format!(" SCORE: {} ", self.score);
        let wave_str = format!(" WAVE: {} ", self.wave);
        let mut lives_str = String::from(" LIVES: ");
        for _ in 0..self.lives {
            lives_str.push('♥');
        }
        lives_str.push(' ');

        self.double_buffer.back.draw_text(1, 0, &score_str, Color::BrightYellow, Color::Reset);
        self.double_buffer.back.draw_text(self.width / 2 - 5, 0, &lives_str, Color::BrightRed, Color::Reset);
        self.double_buffer.back.draw_text(self.width.saturating_sub(wave_str.len() as u16 + 2), 0, &wave_str, Color::BrightCyan, Color::Reset);

        if self.game_over {
            let center_x = self.width / 2;
            let center_y = self.height / 2;
            let banner = "=== GAME OVER ===";
            let sub = "Press ENTER / R to Restart";
            self.double_buffer.back.draw_rect(center_x.saturating_sub(25), center_y.saturating_sub(3), 50, 7, Color::BrightRed);
            self.double_buffer.back.draw_text(center_x.saturating_sub((banner.len() / 2) as u16), center_y.saturating_sub(1), banner, Color::BrightRed, Color::Reset);
            self.double_buffer.back.draw_text(center_x.saturating_sub((sub.len() / 2) as u16), center_y + 1, sub, Color::White, Color::Reset);
        }

        // Return ANSI diff string directly to JS
        self.double_buffer.flush_to_string()
    }

    pub fn last_sound(&self) -> Option<String> {
        self.sound_effect.clone()
    }
}
