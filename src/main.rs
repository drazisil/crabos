#![no_std]      // don't link the Rust standard library
#![no_main]     // disable all Rust-level entry points

#![feature(custom_test_frameworks)]
#![test_runner(crabos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

#[macro_use]
mod serial;

use core::panic::PanicInfo;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crabos::test_panic_handler(info);
}

#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    
    crabos::init(); // new

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); // new

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


