use core::fmt;

static mut LATEST: usize = 0;
const MMIO: usize = 0xb8000;
const COLOR: u8 = 0x07;

pub const ROWS: usize = 25;
pub const COLS: usize = 80;
const MAX: usize = ROWS * COLS;

const SPACE_CHAR: u8 = 32;
const NEWLINE_CHAR: u8 = 10;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VgaColor {
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

pub fn make_color(fg: VgaColor, bg: VgaColor) -> u8 {
    (bg as u8) << 4 | (fg as u8)
}

pub fn char_to_vga(a: u8) {
    unsafe {
        let char: [u8; 2] = [a, COLOR];
        let rel = (MMIO + (LATEST * 2)) as *mut u16;

        rel.write_volatile(u16::from_le_bytes(char));
        LATEST += 1;
    }
}

pub fn str_to_vga(s: &str) {
    let v = s.as_bytes();
    unsafe {
        for i in 0..v.len() {
            if LATEST >= MAX { scroll(); }
            match v[i] {
                NEWLINE_CHAR => LATEST = ((LATEST / COLS) + 1) * COLS,
                _ => char_to_vga(v[i]),
            }
        }
    }
}

pub fn scroll() {
    unsafe {
        for i in COLS..MAX {
            let src = (MMIO + i * 2) as *mut u16;
            let dst = (MMIO + (i - COLS) * 2) as *mut u16;
            dst.write_volatile(src.read_volatile());
        }

        for i in (MAX - COLS)..MAX {
            let dst = (MMIO + i * 2) as *mut u16;
            dst.write_volatile(u16::from_le_bytes([SPACE_CHAR, COLOR]))
        }

        LATEST = LATEST - COLS;
    }
}

pub fn clear_screen() {
    unsafe {
        for i in 0..MAX {
            let ptr = (MMIO + (i * 2)) as *mut u16;
            ptr.write_volatile(u16::from_le_bytes([SPACE_CHAR, COLOR]));
        }
        LATEST = 0;
    }
}

pub fn write_at(row: usize, col: usize, s: &str, color: u8) {
    if row >= ROWS || col >= COLS {
        return;
    }
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        let pos = row * COLS + col + i;
        if pos >= MAX {
            break;
        }
        unsafe {
            let ptr = (MMIO + (pos * 2)) as *mut u16;
            ptr.write_volatile(u16::from_le_bytes([byte, color]));
        }
    }
}

pub fn clear_row(row: usize, color: u8) {
    if row >= ROWS {
        return;
    }
    for col in 0..COLS {
        let pos = row * COLS + col;
        unsafe {
            let ptr = (MMIO + (pos * 2)) as *mut u16;
            ptr.write_volatile(u16::from_le_bytes([SPACE_CHAR, color]));
        }
    }
}

pub fn set_cursor(row: usize, col: usize) {
    unsafe {
        LATEST = row * COLS + col;
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::system::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! clear {
      () => {
          $crate::system::vga::clear_screen()
      };
  }

struct Dummy {}

impl fmt::Write for Dummy {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        str_to_vga(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut d = Dummy { };
    d.write_fmt(args).unwrap();
}

use vga::writers::{Graphics320x200x256, GraphicsWriter, PrimitiveDrawing, Text80x25, TextWriter};

pub const PIXEL_WIDTH: usize = 320;
pub const PIXEL_HEIGHT: usize = 200;

pub mod palette {
    pub const BLACK: u8 = 0;
    pub const BLUE: u8 = 1;
    pub const GREEN: u8 = 2;
    pub const CYAN: u8 = 3;
    pub const RED: u8 = 4;
    pub const MAGENTA: u8 = 5;
    pub const BROWN: u8 = 6;
    pub const LIGHT_GRAY: u8 = 7;
    pub const DARK_GRAY: u8 = 8;
    pub const LIGHT_BLUE: u8 = 9;
    pub const LIGHT_GREEN: u8 = 10;
    pub const LIGHT_CYAN: u8 = 11;
    pub const LIGHT_RED: u8 = 12;
    pub const LIGHT_MAGENTA: u8 = 13;
    pub const YELLOW: u8 = 14;
    pub const WHITE: u8 = 15;
}

pub fn set_mode_320x200x256() {
    let mode = Graphics320x200x256::new();
    mode.set_mode();
}

pub fn put_pixel(x: usize, y: usize, color: u8) {
    let mode = Graphics320x200x256::new();
    mode.set_pixel(x, y, color);
}

pub fn clear_screen_pixel(color: u8) {
    let mode = Graphics320x200x256::new();
    mode.clear_screen(color);
}

pub fn draw_rect(x: usize, y: usize, width: usize, height: usize, color: u8) {
    let mode = Graphics320x200x256::new();
    mode.draw_rect((x, y), (x + width, y + height), color);
}

pub fn draw_line(x1: isize, y1: isize, x2: isize, y2: isize, color: u8) {
    let mode = Graphics320x200x256::new();
    mode.draw_line((x1, y1), (x2, y2), color);
}

pub fn init_default_palette() {
    unsafe {
        use x86_64::instructions::port::Port;

        let default_palette: [(u8, u8, u8); 16] = [
            (0, 0, 0),
            (0, 0, 42),
            (0, 42, 0),
            (0, 42, 42),
            (42, 0, 0),
            (42, 0, 42),
            (42, 21, 0),
            (42, 42, 42),
            (21, 21, 21),
            (21, 21, 63),
            (21, 63, 21),
            (21, 63, 63),
            (63, 21, 21),
            (63, 21, 63),
            (63, 63, 0),
            (63, 63, 63),
        ];

        let mut dac_write: Port<u8> = Port::new(0x3C8);
        let mut dac_data: Port<u8> = Port::new(0x3C9);

        for (i, (r, g, b)) in default_palette.iter().enumerate() {
            dac_write.write(i as u8);
            dac_data.write(*r);
            dac_data.write(*g);
            dac_data.write(*b);
        }
    }
}

pub fn set_text_mode_80x25() {
    use vga::vga::{VideoMode, VGA};
    use vga::fonts::TEXT_8X16_FONT;

    {
        let mut vga = VGA.lock();
        vga.set_video_mode(VideoMode::Mode80x25);
        vga.load_font(&TEXT_8X16_FONT);
    }

    unsafe {
        use x86_64::instructions::port::Port;

        let mut attr_index: Port<u8> = Port::new(0x3C0);
        let mut input_status: Port<u8> = Port::new(0x3DA);

        input_status.read();

        for i in 0..16 {
            attr_index.write(i);
            attr_index.write(i);
        }

        attr_index.write(0x10);
        attr_index.write(0x0C);

        attr_index.write(0x20);
    }

    init_default_palette();

    unsafe {
        for i in 0..MAX {
            let ptr = (MMIO + (i * 2)) as *mut u16;
            ptr.write_volatile(u16::from_le_bytes([SPACE_CHAR, COLOR]));
        }
        LATEST = 0;
    }
}