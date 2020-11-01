#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crabos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

#[macro_use]
mod serial;

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    crabos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crabos::test_panic_handler(info);
}

pub fn kernel_main(_boot_info: &'static BootInfo) -> ! {

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    
    crabos::init();
 
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    crabos::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


