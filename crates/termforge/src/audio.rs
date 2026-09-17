use std::io::{self, Write};
use std::time::{Duration, Instant};

/// Generic pluggable audio driver.
pub trait AudioBackend: Send + Sync {
    fn play(&mut self, sound_name: &str);
}

/// Zero-dependency Terminal Bell audio backend.
pub struct TerminalBellBackend {
    last_bell: Instant,
    cooldown: Duration,
}

impl Default for TerminalBellBackend {
    fn default() -> Self {
        Self {
            last_bell: Instant::now() - Duration::from_secs(1),
            cooldown: Duration::from_millis(150),
        }
    }
}

impl TerminalBellBackend {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            last_bell: Instant::now() - Duration::from_secs(1),
            cooldown,
        }
    }
}

impl AudioBackend for TerminalBellBackend {
    fn play(&mut self, _sound_name: &str) {
        if self.last_bell.elapsed() >= self.cooldown {
            let mut stdout = io::stdout();
            let _ = write!(stdout, "\x07");
            let _ = stdout.flush();
            self.last_bell = Instant::now();
        }
    }
}

/// Frame-level Sound Event Queue.
#[derive(Default)]
pub struct SoundEventQueue {
    events: Vec<String>,
}

impl SoundEventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trigger(&mut self, sound: &str) {
        self.events.push(sound.to_string());
    }

    pub fn flush(&mut self, backend: &mut dyn AudioBackend) {
        for sound in self.events.drain(..) {
            backend.play(&sound);
        }
    }
}
