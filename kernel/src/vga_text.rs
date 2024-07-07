use core::fmt;

const VGA_TEXT_MEMORY: usize = 0xb8000;
const VGA_TEXT_HEIGHT: usize = 25;
const VGA_TEXT_WIDTH: usize = 80;

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

pub fn clear() {
    clear_with_background(Color::Black)
}

pub fn clear_with_background(color: Color) {
    let value = compose_byte(0, Color::Black, color);
    (0..VGA_TEXT_HEIGHT * VGA_TEXT_WIDTH).for_each(|position| vga_memory_write(position, value));
}

fn vga_memory_write(position: usize, value: u16) {
    unsafe {
        core::ptr::write_volatile((VGA_TEXT_MEMORY + 2 * position) as *mut u16, value);
    }
}

fn vga_memory_read(position: usize) -> u16 {
    unsafe { core::ptr::read_volatile((VGA_TEXT_MEMORY + 2 * position) as *const u16) }
}

fn compose_color(foreground: Color, background: Color) -> u8 {
    (background as u8) << 4 | (foreground as u8)
}

fn compose_byte(byte: u8, fg_color: Color, bg_color: Color) -> u16 {
    (byte as u16) | (compose_color(fg_color, bg_color) as u16) << 8
}

struct TerminalWriter {
    position: usize,
    fg_color: Color,
    bg_color: Color,
}

impl TerminalWriter {
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
        let row = self.position / VGA_TEXT_WIDTH;
        let col = self.position % VGA_TEXT_WIDTH;
        if row == VGA_TEXT_HEIGHT - 1 && col == VGA_TEXT_WIDTH - 1 {
            self.new_line()
        }
        vga_memory_write(
            self.position,
            compose_byte(byte, self.fg_color, self.bg_color),
        );
        self.position = (self.position + 1).min(VGA_TEXT_WIDTH * VGA_TEXT_HEIGHT - 1);
    }

    fn new_line(&mut self) {
        let row = self.position / VGA_TEXT_WIDTH;
        if row < VGA_TEXT_HEIGHT - 1 {
            self.position = (row + 1) * VGA_TEXT_WIDTH;
            return;
        }
        self.scroll_content();
        self.position = (VGA_TEXT_HEIGHT - 1) * VGA_TEXT_WIDTH;
    }

    fn scroll_content(&mut self) {
        for pos in 0..VGA_TEXT_WIDTH * (VGA_TEXT_HEIGHT - 1) {
            let val = vga_memory_read(pos + VGA_TEXT_WIDTH);
            vga_memory_write(pos, val);
        }
        // clear last line
        let base = (VGA_TEXT_HEIGHT - 1) * VGA_TEXT_WIDTH;
        for col in 0..VGA_TEXT_WIDTH {
            vga_memory_write(base + col, (self.bg_color as u16) << 4);
        }
    }
}

impl fmt::Write for TerminalWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
static mut TERMINAL_WRITER: TerminalWriter = TerminalWriter {
    position: 0,
    fg_color: Color::White,
    bg_color: Color::Black,
};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_text::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe { TERMINAL_WRITER.write_fmt(args).unwrap() }
}
