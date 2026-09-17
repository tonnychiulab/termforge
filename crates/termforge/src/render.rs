use std::fmt::Write as FmtWrite;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Indexed(u8),
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StyleModifier {
    pub bold: bool,
    pub dim: bool,
    pub underline: bool,
    pub italic: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub modifier: StyleModifier,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: StyleModifier::default(),
        }
    }
}

impl Cell {
    pub const fn new(ch: char) -> Self {
        Self {
            ch,
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: StyleModifier {
                bold: false,
                dim: false,
                underline: false,
                italic: false,
            },
        }
    }

    pub fn with_fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    pub fn with_bold(mut self) -> Self {
        self.modifier.bold = true;
        self
    }
}

/// 2D Grid of character cells with bounds checking.
#[derive(Debug, Clone)]
pub struct Buffer2D {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
}

impl Buffer2D {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            cells: vec![Cell::default(); size],
        }
    }

    #[inline]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u16 {
        self.height
    }

    #[inline]
    pub fn resize(&mut self, width: u16, height: u16) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.cells = vec![Cell::default(); (width as usize) * (height as usize)];
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.cells.fill(Cell::default());
    }

    #[inline]
    fn index(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.width && y < self.height {
            Some((y as usize) * (self.width as usize) + (x as usize))
        } else {
            None
        }
    }

    #[inline]
    pub fn get(&self, x: u16, y: u16) -> Option<Cell> {
        self.index(x, y).map(|idx| self.cells[idx])
    }

    #[inline]
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if let Some(idx) = self.index(x, y) {
            self.cells[idx] = cell;
        }
    }

    pub fn draw_text(&mut self, start_x: u16, start_y: u16, text: &str, fg: Color, bg: Color) {
        let mut curr_x = start_x;
        for ch in text.chars() {
            if curr_x >= self.width {
                break;
            }
            self.set(curr_x, start_y, Cell::new(ch).with_fg(fg).with_bg(bg));
            curr_x += 1;
        }
    }

    pub fn draw_rect(&mut self, x: u16, y: u16, w: u16, h: u16, border_fg: Color) {
        if w == 0 || h == 0 {
            return;
        }
        for col in x..x + w {
            self.set(col, y, Cell::new('─').with_fg(border_fg));
            self.set(col, y + h - 1, Cell::new('─').with_fg(border_fg));
        }
        for row in y..y + h {
            self.set(x, row, Cell::new('│').with_fg(border_fg));
            self.set(x + w - 1, row, Cell::new('│').with_fg(border_fg));
        }
        self.set(x, y, Cell::new('┌').with_fg(border_fg));
        self.set(x + w - 1, y, Cell::new('┐').with_fg(border_fg));
        self.set(x, y + h - 1, Cell::new('└').with_fg(border_fg));
        self.set(x + w - 1, y + h - 1, Cell::new('┘').with_fg(border_fg));
    }
}

/// Double-buffering engine with minimal ANSI diff serialization.
pub struct DoubleBuffer {
    pub front: Buffer2D,
    pub back: Buffer2D,
    command_buffer: String,
}

impl DoubleBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            front: Buffer2D::new(width, height),
            back: Buffer2D::new(width, height),
            command_buffer: String::with_capacity(32 * 1024),
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.front.resize(width, height);
        self.back.resize(width, height);
    }

    /// Compute delta between front and back buffers, write minimal ANSI escapes, and swap.
    pub fn flush<W: Write>(&mut self, target: &mut W) -> io::Result<usize> {
        self.command_buffer.clear();

        let width = self.back.width;
        let height = self.back.height;

        let mut last_cursor: Option<(u16, u16)> = None;
        let mut last_fg = Color::Reset;
        let mut last_bg = Color::Reset;
        let mut bytes_written = 0;

        for y in 0..height {
            for x in 0..width {
                let idx = (y as usize) * (width as usize) + (x as usize);
                let back_cell = self.back.cells[idx];
                let front_cell = self.front.cells[idx];

                if back_cell != front_cell {
                    // Update front cell
                    self.front.cells[idx] = back_cell;

                    // Cursor positioning optimization
                    let needs_jump = match last_cursor {
                        Some((lx, ly)) => ly != y || lx + 1 != x,
                        None => true,
                    };

                    if needs_jump {
                        // ANSI 1-indexed cursor position: \x1b[{row};{col}H
                        let _ = write!(self.command_buffer, "\x1b[{};{}H", y + 1, x + 1);
                    }
                    last_cursor = Some((x, y));

                    // Style & color optimization
                    if back_cell.fg != last_fg {
                        self.append_fg_color(back_cell.fg);
                        last_fg = back_cell.fg;
                    }
                    if back_cell.bg != last_bg {
                        self.append_bg_color(back_cell.bg);
                        last_bg = back_cell.bg;
                    }

                    self.command_buffer.push(back_cell.ch);
                }
            }
        }

        if !self.command_buffer.is_empty() {
            // Reset color at the end of frame batch
            self.command_buffer.push_str("\x1b[0m");
            target.write_all(self.command_buffer.as_bytes())?;
            target.flush()?;
            bytes_written = self.command_buffer.len();
        }

        Ok(bytes_written)
    }

    /// Compute delta between front and back buffers and return ANSI string directly (ideal for WASM).
    pub fn flush_to_string(&mut self) -> String {
        self.command_buffer.clear();

        let width = self.back.width;
        let height = self.back.height;

        let mut last_cursor: Option<(u16, u16)> = None;
        let mut last_fg = Color::Reset;
        let mut last_bg = Color::Reset;

        for y in 0..height {
            for x in 0..width {
                let idx = (y as usize) * (width as usize) + (x as usize);
                let back_cell = self.back.cells[idx];
                let front_cell = self.front.cells[idx];

                if back_cell != front_cell {
                    self.front.cells[idx] = back_cell;

                    let needs_jump = match last_cursor {
                        Some((lx, ly)) => ly != y || lx + 1 != x,
                        None => true,
                    };

                    if needs_jump {
                        let _ = write!(self.command_buffer, "\x1b[{};{}H", y + 1, x + 1);
                    }
                    last_cursor = Some((x, y));

                    if back_cell.fg != last_fg {
                        self.append_fg_color(back_cell.fg);
                        last_fg = back_cell.fg;
                    }
                    if back_cell.bg != last_bg {
                        self.append_bg_color(back_cell.bg);
                        last_bg = back_cell.bg;
                    }

                    self.command_buffer.push(back_cell.ch);
                }
            }
        }

        if !self.command_buffer.is_empty() {
            self.command_buffer.push_str("\x1b[0m");
        }

        self.command_buffer.clone()
    }

    fn append_fg_color(&mut self, color: Color) {
        match color {
            Color::Reset => self.command_buffer.push_str("\x1b[39m"),
            Color::Black => self.command_buffer.push_str("\x1b[30m"),
            Color::Red => self.command_buffer.push_str("\x1b[31m"),
            Color::Green => self.command_buffer.push_str("\x1b[32m"),
            Color::Yellow => self.command_buffer.push_str("\x1b[33m"),
            Color::Blue => self.command_buffer.push_str("\x1b[34m"),
            Color::Magenta => self.command_buffer.push_str("\x1b[35m"),
            Color::Cyan => self.command_buffer.push_str("\x1b[36m"),
            Color::White => self.command_buffer.push_str("\x1b[37m"),
            Color::BrightBlack => self.command_buffer.push_str("\x1b[90m"),
            Color::BrightRed => self.command_buffer.push_str("\x1b[91m"),
            Color::BrightGreen => self.command_buffer.push_str("\x1b[92m"),
            Color::BrightYellow => self.command_buffer.push_str("\x1b[93m"),
            Color::BrightBlue => self.command_buffer.push_str("\x1b[94m"),
            Color::BrightMagenta => self.command_buffer.push_str("\x1b[95m"),
            Color::BrightCyan => self.command_buffer.push_str("\x1b[96m"),
            Color::BrightWhite => self.command_buffer.push_str("\x1b[97m"),
            Color::Indexed(n) => {
                let _ = write!(self.command_buffer, "\x1b[38;5;{}m", n);
            }
            Color::Rgb(r, g, b) => {
                let _ = write!(self.command_buffer, "\x1b[38;2;{};{};{}m", r, g, b);
            }
        }
    }

    fn append_bg_color(&mut self, color: Color) {
        match color {
            Color::Reset => self.command_buffer.push_str("\x1b[49m"),
            Color::Black => self.command_buffer.push_str("\x1b[40m"),
            Color::Red => self.command_buffer.push_str("\x1b[41m"),
            Color::Green => self.command_buffer.push_str("\x1b[42m"),
            Color::Yellow => self.command_buffer.push_str("\x1b[43m"),
            Color::Blue => self.command_buffer.push_str("\x1b[44m"),
            Color::Magenta => self.command_buffer.push_str("\x1b[45m"),
            Color::Cyan => self.command_buffer.push_str("\x1b[46m"),
            Color::White => self.command_buffer.push_str("\x1b[47m"),
            Color::BrightBlack => self.command_buffer.push_str("\x1b[100m"),
            Color::BrightRed => self.command_buffer.push_str("\x1b[101m"),
            Color::BrightGreen => self.command_buffer.push_str("\x1b[102m"),
            Color::BrightYellow => self.command_buffer.push_str("\x1b[103m"),
            Color::BrightBlue => self.command_buffer.push_str("\x1b[104m"),
            Color::BrightMagenta => self.command_buffer.push_str("\x1b[105m"),
            Color::BrightCyan => self.command_buffer.push_str("\x1b[106m"),
            Color::BrightWhite => self.command_buffer.push_str("\x1b[107m"),
            Color::Indexed(n) => {
                let _ = write!(self.command_buffer, "\x1b[48;5;{}m", n);
            }
            Color::Rgb(r, g, b) => {
                let _ = write!(self.command_buffer, "\x1b[48;2;{};{};{}m", r, g, b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_draw_and_diff() {
        let mut db = DoubleBuffer::new(10, 5);
        db.back.draw_text(0, 0, "HI", Color::Green, Color::Reset);

        let mut output = Vec::new();
        let bytes = db.flush(&mut output).unwrap();
        assert!(bytes > 0);

        // Second flush without modification should produce 0 bytes (zero-overhead dirty check)
        let mut output2 = Vec::new();
        let bytes2 = db.flush(&mut output2).unwrap();
        assert_eq!(bytes2, 0);
    }
}
