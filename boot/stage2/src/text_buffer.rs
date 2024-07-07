use core::fmt;

const BUFFER_START: usize = 0xb8000;
const BUFFER_ROWS: usize = 25;
const BUFFER_COLS: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

fn pack_color(foreground: Color, background: Color) -> u8 {
    (background as u8) << 4 | (foreground as u8)
}

pub fn clear() {
    clear_with_background(Color::Black)
}

pub fn clear_with_background(color: Color) {
    (0..BUFFER_ROWS * BUFFER_COLS).for_each(|offset| unsafe {
        core::ptr::write_volatile((BUFFER_START + 2 * offset) as *mut usize, 0);
        core::ptr::write_volatile(
            (BUFFER_START + 2 * offset + 1) as *mut u8,
            (color as u8) << 4,
        );
    })
}

fn buffer_write(position: usize, value: u16) {
    unsafe {
        core::ptr::write_volatile((BUFFER_START + 2 * position) as *mut u16, value);
    }
}

fn buffer_read(position: usize) -> u16 {
    unsafe { core::ptr::read_volatile((BUFFER_START + 2 * position) as *const u16) }
}

pub struct Writer {
    position: usize,
    fg_color: Color,
    bg_color: Color,
}

impl Writer {
    pub fn write_string(&mut self, string: &str) {
        string.bytes().for_each(|byte| self.write_byte(byte))
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            0x20..=0x7e => self.write_ascii_byte(byte),
            _ => self.write_ascii_byte(0x7e),
        }
    }

    fn write_ascii_byte(&mut self, byte: u8) {
        let row = self.position / BUFFER_COLS;
        let col = self.position % BUFFER_COLS;
        if row == BUFFER_ROWS - 1 && col == BUFFER_COLS - 1 {
            self.new_line()
        }
        buffer_write(
            self.position,
            (byte as u16) | (pack_color(self.fg_color, self.bg_color) as u16) << 8,
        );
        self.position = (self.position + 1).min(BUFFER_COLS * BUFFER_ROWS - 1);
    }

    fn new_line(&mut self) {
        let row = self.position / BUFFER_COLS;
        if row < BUFFER_ROWS - 1 {
            self.position = (row + 1) * BUFFER_COLS;
            return;
        }
        self.scroll_content();
        self.position = (BUFFER_ROWS - 1) * BUFFER_COLS;
    }

    fn scroll_content(&mut self) {
        for pos in 0..BUFFER_COLS * (BUFFER_ROWS - 1) {
            let val = buffer_read(pos + BUFFER_COLS);
            buffer_write(pos, val);
        }
        // clear last line
        let base = (BUFFER_ROWS - 1) * BUFFER_COLS;
        for col in 0..BUFFER_COLS {
            buffer_write(base + col, (self.bg_color as u16) << 4);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
pub static mut WRITER: Writer = Writer {
    position: 0,
    fg_color: Color::White,
    bg_color: Color::Black,
};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::text_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe { WRITER.write_fmt(args).unwrap() }
}
