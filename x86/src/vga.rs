use core::fmt;

static mut LATEST: usize = 0;
const MMIO: usize = 0xb8000;
const COLOR: u8 = 0xF;

const ROWS: usize = 25;
const COLS: usize = 80;
const MAX: usize = ROWS * COLS;

const SPACE_CHAR: u8 = 32;
const NEWLINE_CHAR: u8 = 10;

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
            if LATEST > MAX { scroll(); }
            match v[i] {
                NEWLINE_CHAR => LATEST = ((LATEST / COLS) + 1) * COLS,
                _ => char_to_vga(v[i]),
            }
        }
    }
}

pub(crate) fn scroll() {
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

        LATEST = LATEST - ROWS;
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
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