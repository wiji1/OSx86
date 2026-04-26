const PORT_LOCATION: u16 = 0x3F8;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub const fn code(&self) -> &str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
        }
    }

    pub const fn reset() -> &'static str {
        "\x1b[0m"
    }
}

#[macro_export]
macro_rules! serial_print {
    ($color:path, $($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*), Some($color));
    };
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*), None);
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($color:path, $fmt:expr) => ($crate::serial_print!($color, concat!($fmt, "\n")));
    ($color:path, $fmt:expr, $($arg:tt)*) => ($crate::serial_print!($color, concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn _print(args: core::fmt::Arguments, color: Option<Color>) {
    use core::fmt::Write;

    unsafe {
        let mut serial_port = uart_16550::SerialPort::new(PORT_LOCATION);
        serial_port.init();

        if let Some(c) = color {
            serial_port.write_str(c.code()).unwrap();
        }

        serial_port.write_fmt(args).unwrap();

        if color.is_some() {
            serial_port.write_str(Color::reset()).unwrap();
        }
    }
}