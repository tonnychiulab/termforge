use std::io::{self, Stdout, Write};
use std::panic;
use std::sync::atomic::{AtomicBool, Ordering};
use termion::raw::{IntoRawMode, RawTerminal};

static TERMINAL_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// RAII Guard that manages raw mode, alternate screen, and cursor visibility.
/// Restores canonical terminal state automatically on Drop or Panic.
pub struct TerminalGuard {
    raw_terminal: Option<RawTerminal<Stdout>>,
}

impl TerminalGuard {
    pub fn new() -> io::Result<Self> {
        // Register panic hook to restore terminal if a panic occurs
        Self::install_panic_hook();

        let mut raw = io::stdout().into_raw_mode()?;
        
        // Enter alternate screen and hide cursor via standard ANSI sequences
        write!(
            raw,
            "{}{}",
            termion::screen::ToAlternateScreen,
            termion::cursor::Hide
        )?;
        raw.flush()?;

        TERMINAL_INITIALIZED.store(true, Ordering::SeqCst);

        Ok(Self {
            raw_terminal: Some(raw),
        })
    }

    fn install_panic_hook() {
        let default_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            // Restore terminal state before printing panic info
            Self::restore_raw_terminal();
            default_hook(panic_info);
        }));
    }

    fn restore_raw_terminal() {
        if TERMINAL_INITIALIZED.swap(false, Ordering::SeqCst) {
            let mut stdout = io::stdout();
            // Show cursor and switch back to main screen buffer
            let _ = write!(
                stdout,
                "{}{}{}",
                termion::cursor::Show,
                termion::screen::ToMainScreen,
                termion::style::Reset
            );
            let _ = stdout.flush();
        }
    }

    /// Query current terminal dimensions (columns, rows).
    pub fn size() -> io::Result<(u16, u16)> {
        termion::terminal_size()
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if let Some(mut raw) = self.raw_terminal.take() {
            let _ = write!(
                raw,
                "{}{}{}",
                termion::cursor::Show,
                termion::screen::ToMainScreen,
                termion::style::Reset
            );
            let _ = raw.flush();
        }
        TERMINAL_INITIALIZED.store(false, Ordering::SeqCst);
    }
}
