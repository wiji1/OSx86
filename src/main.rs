#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

mod system;
mod snake;
mod menu;
mod apps;

#[unsafe(no_mangle)]
pub fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    system::interrupts::init_idt();
    system::gdt::init_gdt();

    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { system::memory::init(offset) };
    let mut frame_allocator = unsafe { system::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    system::allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();

    system::vga::set_text_mode_80x25();

    static SNAKE_APP: snake::game::SnakeGame = snake::game::SnakeGame;
    static HELLO_APP: apps::HelloWorldApp = apps::HelloWorldApp;
    static INFO_APP: apps::SystemInfoApp = apps::SystemInfoApp;

    static MENU_ITEMS: &[&dyn menu::Application] = &[
        &SNAKE_APP,
        &HELLO_APP,
        &INFO_APP,
    ];

    let main_menu = menu::Menu::new("Select an Application", MENU_ITEMS);

    loop {
        main_menu.run();
    }
}

pub fn halt() -> ! {
    loop { x86_64::instructions::hlt(); }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    serial_println!(system::serial::Color::Red, "[PANIC] {}", _info);

    #[cfg(test)]
    {
        system::exit::exit_qemu(system::exit::QemuExitCode::Failed);
    }

    #[cfg(not(test))]
    halt()
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!(system::serial::Color::Cyan, "Running {} tests", tests.len());
    for test in tests {
        test();
    }
    serial_println!(system::serial::Color::Green, "All tests passed!");
    system::exit::exit_qemu(system::exit::QemuExitCode::Success);
}