use crate::system::keyboard;
use crate::system::vga::{self, make_color, VgaColor};
use pc_keyboard::{DecodedKey, KeyCode};
use spin::Mutex;

pub trait Application: Sync {
    fn name(&self) -> &'static str;
    fn run(&self);
}

static MENU_STATE: Mutex<MenuState> = Mutex::new(MenuState {
    selected: 0,
    num_items: 0,
    running: false,
    title: "",
    items_ptr: core::ptr::null(),
});

struct MenuState {
    selected: usize,
    num_items: usize,
    running: bool,
    title: &'static str,
    items_ptr: *const &'static dyn Application,
}

unsafe impl Send for MenuState {}

pub struct Menu {
    title: &'static str,
    items: &'static [&'static dyn Application],
}

impl Menu {
    pub const fn new(title: &'static str, items: &'static [&'static dyn Application]) -> Self {
        Menu { title, items }
    }

    pub fn run(&self) {
        loop {
            {
                let mut state = MENU_STATE.lock();
                state.selected = 0;
                state.num_items = self.items.len();
                state.running = true;
                state.title = self.title;
                state.items_ptr = self.items.as_ptr();
            }

            keyboard::set_key_handler(menu_key_handler);

            draw_menu();

            x86_64::instructions::interrupts::enable();

            loop {
                let state = MENU_STATE.lock();
                if !state.running {
                    let selected = state.selected;
                    drop(state);

                    keyboard::clear_key_handler();
                    if selected < self.items.len() {
                        self.items[selected].run();
                    }

                    break;
                }
                drop(state);
                x86_64::instructions::interrupts::enable_and_hlt();
            }
        }
    }
}

fn draw_menu() {
    let state = MENU_STATE.lock();

    vga::clear_screen();

    let normal_color = make_color(VgaColor::White, VgaColor::Black);
    let highlight_color = make_color(VgaColor::Black, VgaColor::Cyan);

    vga::write_at(2, 5, state.title, make_color(VgaColor::Yellow, VgaColor::Black));

    let start_row = 5;
    for i in 0..state.num_items {
        let row = start_row + i;
        let color = if i == state.selected {
            highlight_color
        } else {
            normal_color
        };

        vga::clear_row(row, color);

        unsafe {
            let item = *state.items_ptr.add(i);
            vga::write_at(row, 10, item.name(), color);
        }
    }

    vga::write_at(start_row + state.num_items + 2, 5,
        "Arrow Keys: Navigate  |  Enter: Select",
        make_color(VgaColor::DarkGray, VgaColor::Black));
}

fn menu_key_handler(key: DecodedKey) {
    match key {
        DecodedKey::RawKey(KeyCode::ArrowUp) => {
            let mut state = MENU_STATE.lock();
            if state.selected > 0 {
                state.selected -= 1;
            }
            drop(state);
            draw_menu();
        }
        DecodedKey::RawKey(KeyCode::ArrowDown) => {
            let mut state = MENU_STATE.lock();
            if state.selected + 1 < state.num_items {
                state.selected += 1;
            }
            drop(state);
            draw_menu();
        }
        DecodedKey::RawKey(KeyCode::Return) | DecodedKey::Unicode('\n') => {
            let mut state = MENU_STATE.lock();
            state.running = false;
        }
        _ => {}
    }
}
