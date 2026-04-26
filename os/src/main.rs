#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use alloc::boxed::Box;

mod vga;
mod serial;

#[cfg(test)]
mod exit;
mod interrupts;
mod gdt;
mod memory;
mod allocator;

#[unsafe(no_mangle)]
pub fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    interrupts::init_idt();
    gdt::init_gdt();

    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();

    clear!();

    let b = Box::new(371);
    println!("heap: {:?}", b);

    #[cfg(test)]
    test_main();

    halt()
}

pub fn halt() -> ! {
    loop { x86_64::instructions::hlt(); }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    serial_println!(serial::Color::Red, "[PANIC] {}", _info);

    #[cfg(test)]
    {
        exit::exit_qemu(exit::QemuExitCode::Failed);
    }

    #[cfg(not(test))]
    halt()
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!(serial::Color::Cyan, "Running {} tests", tests.len());
    for test in tests {
        test();
    }
    serial_println!(serial::Color::Green, "All tests passed!");
    exit::exit_qemu(exit::QemuExitCode::Success);
}

#[test_case]
fn hi() {
    serial_println!(serial::Color::Green, "test hi... ok");
}

#[test_case]
fn bye() {
    serial_println!(serial::Color::Green, "test bye... ok");
}