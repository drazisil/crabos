#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]
#![feature(asm)]

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

pub mod serial;

pub mod sysinfo;

mod register;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }; // new
    // interrupts::enable();
}

#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {

    vga_buffer::clear_screen();

    // sysinfo::dump_sysinfo(multiboot_information_address);

    init();

    let eax: u32;
    let ebx: u32;
    let ecx: u32;
    let edx: u32;
    unsafe {
        asm! (
            "mov eax, 0x1",
            "cpuid",
            out("eax") eax,
            out("ebx") ebx,
            out("ecx") ecx,
            out("edx") edx,
        );
    }

    kdebug!("cpuid feature information");
    kdebug!("eax is {}", eax);
    kdebug!("ebx is {}", ebx);
    kdebug!("ecx is {}", ecx);
    kdebug!("edx is {}", edx);

    let eflags = register::eflags();
    
    kdebug!("eflags: {}", eflags);

    let features = sysinfo::cpu_features();

    kdebug!("cpuid feature information 2");
    kdebug!("eax is {}", features.eax);
    kdebug!("ebx is {}", features.ebx);
    kdebug!("ecx is {}", features.ecx);
    kdebug!("edx is {}", features.edx);

    kdebug!("onload_apic: {}", sysinfo::check_onboard_apic());
    kdebug!("onload_apic: {}", sysinfo::check_onboard_apic()); 

    println!("Hello, kernel!");

    hlt_loop();
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

