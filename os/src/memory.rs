use x86_64::registers::control::Cr3;
use x86_64::structures::paging::FrameAllocator;
use x86_64::structures::paging::PageTable;
use x86_64::structures::paging::PhysFrame;
use x86_64::structures::paging::Size4KiB;
use x86_64::VirtAddr;

pub unsafe fn init(offset: VirtAddr, ) -> x86_64::structures::paging::OffsetPageTable<'static> {
    unsafe {
        let l4 = active_level_4_table(offset);
        x86_64::structures::paging::OffsetPageTable::new(l4, offset)
    }
}

pub unsafe fn active_level_4_table(offset: x86_64::VirtAddr) -> &'static mut PageTable {
    unsafe {
        let (frame, _) = Cr3::read();

        let phys = frame.start_address();
        let virt = offset + phys.as_u64();
        let ptr: *mut PageTable = virt.as_mut_ptr();

        &mut *ptr
    }
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static bootloader::bootinfo::MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static bootloader::bootinfo::MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == bootloader::bootinfo::MemoryRegionType::Usable);
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r: core::ops::Range<u64>| r.step_by(4096));
        frame_addresses
            .map(|addr| PhysFrame::containing_address(x86_64::PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB>
for BootInfoFrameAllocator
{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}