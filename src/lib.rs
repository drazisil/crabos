#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod interrupts;

use core::panic::PanicInfo;

// Since this is a library, we need to import the lazy_static macro here
#[macro_use(lazy_static)]
extern crate lazy_static;

// Import the vga_buffer file
mod vga_buffer;

use crate::vga_buffer::WRITER;

pub fn init() {
    interrupts::init_idt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    // init();

    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hlt_loop()
}