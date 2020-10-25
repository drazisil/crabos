#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Since this is a library, we need to import the lazy_static macro here
#[macro_use(lazy_static)]
extern crate lazy_static;

// Import the vga_buffer file
mod vga_buffer;

use crate::vga_buffer::WRITER;


#[no_mangle]
pub extern "C" fn _start() {
    cls!();
    println!("Hello World{}", "!");

    // loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {

    }
}