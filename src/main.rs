#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crabos::test_runner)]
#![reexport_test_harness_main = "test_main"]

// #[macro_use(lazy_static)]
// extern crate lazy_static;

// #[cfg(test)]
// fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//     }
//
//     crabos::exit_qemu(crabos::QemuExitCode::Success);
// }

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crabos::println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crabos::serial_println!("[failed]\n");
    crabos::serial_println!("Error: {}\n", info);
    crabos::exit_qemu(crabos::QemuExitCode::Failed);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    crabos::println!("Hello World{}", "!");

    #[cfg(test)]
        test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    crabos::serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    crabos::serial_println!("[ok]");
}
