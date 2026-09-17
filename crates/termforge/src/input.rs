use std::collections::HashSet;
use std::io;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
pub use termion::event::Key;
use termion::input::TermRead;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    KeyDown(Key),
    KeyUp(Key),
}

/// Non-blocking keyboard state and event tracker.
pub struct InputState {
    pressed_keys: HashSet<Key>,
    just_pressed_keys: HashSet<Key>,
    just_released_keys: HashSet<Key>,
    event_receiver: Option<Receiver<Key>>,
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            just_pressed_keys: HashSet::new(),
            just_released_keys: HashSet::new(),
            event_receiver: None,
        }
    }

    /// Spawns a background thread reading keys from stdin in raw mode.
    pub fn start_input_thread(&mut self) {
        let (sender, receiver) = channel();
        self.event_receiver = Some(receiver);

        thread::spawn(move || {
            let stdin = io::stdin();
            for key in stdin.keys().flatten() {
                if sender.send(key).is_err() {
                    break;
                }
            }
        });
    }

    /// Poll incoming keys and update state transitions for the current frame.
    pub fn update(&mut self) {
        self.just_pressed_keys.clear();
        self.just_released_keys.clear();

        if let Some(ref receiver) = self.event_receiver {
            while let Ok(key) = receiver.try_recv() {
                if !self.pressed_keys.contains(&key) {
                    self.just_pressed_keys.insert(key);
                    self.pressed_keys.insert(key);
                }
            }
        }
    }

    #[inline]
    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    #[inline]
    pub fn just_pressed(&self, key: Key) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    #[inline]
    pub fn release_key(&mut self, key: Key) {
        if self.pressed_keys.remove(&key) {
            self.just_released_keys.insert(key);
        }
    }

    #[inline]
    pub fn clear_all(&mut self) {
        self.pressed_keys.clear();
        self.just_pressed_keys.clear();
        self.just_released_keys.clear();
    }
}
