#![allow(static_mut_refs)]
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::instructions::segmentation::CS;
use x86_64::instructions::tables::load_tss;
use x86_64::instructions::segmentation::Segment;

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

const STACK_SIZE: usize = 4096 * 5;
static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

pub fn init_gdt() {
    unsafe {
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = x86_64::VirtAddr::from_ptr(&raw const STACK) + STACK_SIZE as u64;

        let kcs = GDT.append(x86_64::structures::gdt::Descriptor::kernel_code_segment());
        let tss = GDT.append(x86_64::structures::gdt::Descriptor::tss_segment(&TSS));

        GDT.load();

        CS::set_reg(kcs);
        load_tss(tss);
    }
}