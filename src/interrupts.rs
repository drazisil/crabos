use crate::{kdebug,kerror,println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use x86_64::structures::idt::PageFaultErrorCode;
use crate::hlt_loop;
use pic8259_simple::ChainedPics;
use spin;
use crate::gdt;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // idt.divide_error.set_handler_fn(fallback_fault_handler);
        // idt.non_maskable_interrupt.set_handler_fn(fallback_fault_handler);
        // idt.overflow.set_handler_fn(fallback_fault_handler);
        // idt.bound_range_exceeded.set_handler_fn(fallback_fault_handler);
        // idt.invalid_opcode.set_handler_fn(fallback_fault_handler);
        // idt.device_not_available.set_handler_fn(fallback_fault_handler);
        // idt.x87_floating_point.set_handler_fn(fallback_fault_handler);
        // idt.virtualization.set_handler_fn(fallback_fault_handler);
        // idt.invalid_tss.set_handler_fn(fallback_fault_handler_with_err);
        // idt.segment_not_present.set_handler_fn(fallback_fault_handler_with_err);
        // idt.stack_segment_fault.set_handler_fn(fallback_fault_handler_with_err);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }
        // idt.page_fault.set_handler_fn(page_fault_handler); // new
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler); // new
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
// new
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: &mut InterruptStackFrame)
{
    kdebug!("Timer .");
    println!("Timer .");

    // unsafe {
    //     PICS.lock()
    //         .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    // }
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: &mut InterruptStackFrame, error_code: u64)
{
    panic!("EXCEPTION: GPE ({})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn fallback_fault_handler(
    stack_frame: &mut InterruptStackFrame)
{
    panic!("EXCEPTION: UNHANDLED INTERRUPT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn fallback_fault_handler_with_err(
    stack_frame: &mut InterruptStackFrame, error_code: u64)
{
    panic!("EXCEPTION: UNHANDLED INTERRUPT ({})\n{:#?}", error_code, stack_frame);
}

#[inline]
pub fn enable() {
    unsafe {
        asm!(
            "sti"
        );
    }

}