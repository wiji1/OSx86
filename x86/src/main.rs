#![no_std]
#![no_main]
#![allow(unconditional_recursion)]

const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let message = b"Hello world! This is a cool program that has rainbow text. I wonder how long it can go.";

    for (i, &byte) in message.iter().enumerate() {
        let color_ord: u8 = ((i % 15) + 1) as u8;
        let buffer: [u8; 2] = [byte, color_ord];

        unsafe {
            VGA_BUFFER.add(i).write(u16::from_le_bytes(buffer));
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}