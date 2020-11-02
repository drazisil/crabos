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

use crabos::task::Task;
use crabos::task::executor::Executor;

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

use crabos::task::keyboard;

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

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {


    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    
    crabos::init();

    use crabos::memory::{self, BootInfoFrameAllocator};
    use crabos::allocator; // new import
    use x86_64::VirtAddr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
 
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    crabos::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}


