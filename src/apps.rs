use crate::menu::Application;
use crate::system::vga::{self, VgaColor, make_color};

pub struct HelloWorldApp;

impl Application for HelloWorldApp {
    fn name(&self) -> &'static str {
        "Hello World Demo"
    }

    fn run(&self) {
        vga::clear_screen();
        vga::write_at(10, 20, "Hello from the Hello World app!",
            make_color(VgaColor::LightCyan, VgaColor::Black));
        vga::write_at(12, 20, "Press any key to return to menu...",
            make_color(VgaColor::LightGray, VgaColor::Black));

        use crate::system::keyboard;
        use pc_keyboard::DecodedKey;
        use spin::Mutex;

        static PRESSED: Mutex<bool> = Mutex::new(false);

        *PRESSED.lock() = false;

        keyboard::set_key_handler(|_key: DecodedKey| {
            *PRESSED.lock() = true;
        });

        loop {
            if *PRESSED.lock() {
                break;
            }
            x86_64::instructions::hlt();
        }

        keyboard::clear_key_handler();
        vga::set_text_mode_80x25();
    }
}

pub struct SystemInfoApp;

impl Application for SystemInfoApp {
    fn name(&self) -> &'static str {
        "System Information"
    }

    fn run(&self) {
        vga::clear_screen();
        vga::write_at(2, 5, "System Information",
            make_color(VgaColor::Yellow, VgaColor::Black));
        vga::write_at(4, 5, "OS: Custom Rust OS",
            make_color(VgaColor::White, VgaColor::Black));
        vga::write_at(5, 5, "Architecture: x86_64",
            make_color(VgaColor::White, VgaColor::Black));
        vga::write_at(6, 5, "VGA Mode: 80x25 Text",
            make_color(VgaColor::White, VgaColor::Black));

        vga::write_at(10, 5, "Press any key to return to menu...",
            make_color(VgaColor::DarkGray, VgaColor::Black));

        use crate::system::keyboard;
        use pc_keyboard::DecodedKey;
        use spin::Mutex;

        static PRESSED: Mutex<bool> = Mutex::new(false);

        *PRESSED.lock() = false;

        keyboard::set_key_handler(|_key: DecodedKey| {
            *PRESSED.lock() = true;
        });

        loop {
            if *PRESSED.lock() {
                break;
            }
            x86_64::instructions::hlt();
        }

        keyboard::clear_key_handler();
    }
}
