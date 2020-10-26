#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]

pub mod interrupts;

use core::panic::PanicInfo;

// Since this is a library, we need to import the lazy_static macro here
#[macro_use(lazy_static)]
extern crate lazy_static;

// Import the vga_buffer file
mod vga_buffer;

use crate::vga_buffer::WRITER;

pub mod gdt;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }; // new
}

#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
                 area.base_addr, area.length);
    }

    init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    // stack_overflow();

    // // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

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

