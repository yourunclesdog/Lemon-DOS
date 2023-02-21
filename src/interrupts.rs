use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::serial_println;
use lazy_static::lazy_static;

//usage of lazy_static ensures that this is only called once
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_hander);
        idt.double_fault.set_handler_fn(doublefault_hander);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

//the hander for breakpoint interrupts
extern "x86-interrupt" fn breakpoint_hander(stack_frame: InterruptStackFrame) {
    serial_println!("Exception - Breakpoint\n{:#?}", stack_frame);
}

//the hander for double fault interrupts
extern "x86-interrupt" fn doublefault_hander(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Exception - Double Fault\n{:#?}", stack_frame);
}
