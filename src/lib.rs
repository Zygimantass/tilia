#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
pub mod allocator;
pub mod debug_printer;
pub mod devices;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod vga_buffer;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn debugln(s: &str) {
    use core::fmt::Write;
    write!(debug_printer::DEBUG_WRITER.lock(), "[tilia] [debug] {}\n", s).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        vga_buffer::WRITER.lock().write_fmt(args).unwrap();
        debug_printer::DEBUG_WRITER.lock().write_fmt(args).unwrap();
    });
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("alloc error: {:#?}", layout);
}

extern crate alloc;

use alloc::fmt;
