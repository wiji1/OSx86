#![no_std]
#![no_main]

mod vga;

#[unsafe(no_mangle)]
pub fn _start() -> ! {
    println!("Hello, world!\nHi\nNew line!");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}