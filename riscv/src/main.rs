#![no_std]
#![no_main]
#![allow(unconditional_recursion)]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    unsafe {
        let uart: *mut u8 = 0x10010000 as *mut u8;
        for &b in b"hello\n" {
            uart.write_volatile(b);
        }
    }
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic(info)
}