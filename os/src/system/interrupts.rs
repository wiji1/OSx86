#![allow(static_mut_refs)]

use x86_64::structures::idt::PageFaultErrorCode;
use core::ptr::addr_of_mut;
use pc_keyboard::layouts;
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::InterruptDescriptorTable;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        let idt = &mut *addr_of_mut!(IDT);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(crate::system::gdt::DOUBLE_FAULT_IST_INDEX as u16);
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt.load();

        IDT[InterruptIndex::Timer as u8].set_handler_fn(timer_handler);
        IDT[InterruptIndex::Keyboard as u8].set_handler_fn(keyboard_interrupt_handler);


        PICS.initialize();
        x86_64::instructions::interrupts::enable();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    assert_eq!(error_code, 0);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_handler (_stack_frame: InterruptStackFrame) {
    crate::system::timer::handle_tick();
    unsafe { PICS.notify_end_of_interrupt(InterruptIndex::Timer as u8) };
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;

    static KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            crate::system::keyboard::handle_key(key);
        }
    }

    unsafe {
        PICS.notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    crate::println!("EXCEPTION: PAGE FAULT");
    crate::println!("Accessed Address: {:?}", x86_64::registers::control::Cr2::read());
    crate::println!("Error Code: {:?}", error_code);
    crate::println!("{:#?}", stack_frame);
    crate::halt();
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static mut PICS: pic8259::ChainedPics = unsafe {
    pic8259::ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
};

#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard
}