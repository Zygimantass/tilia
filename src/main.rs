#![no_std]
#![no_main]
extern crate alloc;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use tilia::debugln;
use tilia::println;
entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    tilia::init();
    init_memory(boot_info);

    let _devices = tilia::devices::pci::PCIEnumerator::enumerate();
    tilia::hlt_loop();
}

fn init_memory(boot_info: &'static BootInfo) {
    use tilia::allocator;
    use tilia::memory;
    use x86_64::VirtAddr;

    // initalize paging
    debugln("initializing paging");
    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut page_mapper = unsafe { memory::init(physical_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    debugln("initializing heap allocator");

    // initialize heap allocator
    allocator::init_heap(&mut page_mapper, &mut frame_allocator).expect("failed to initalize heap");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}
