#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]

pub mod interrupts;

use core::panic::PanicInfo;

// Since this is a library, we need to import the lazy_static macro here
#[macro_use(lazy_static)]
extern crate lazy_static;

// Import the vga_buffer file
#[macro_use]
pub mod vga_buffer;

pub mod gdt;

pub mod memory;

mod sysinfo;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }; // new
}

#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    sysinfo::dump_sysinfo(multiboot_information_address);

    init();

    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC: {}", _info);
    hlt_loop()
}

