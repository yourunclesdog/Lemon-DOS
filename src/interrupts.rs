use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::serial_println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_hander);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_hander(stack_frame: InterruptStackFrame) {
    serial_println!("Exception - Breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn doublefault_hander(stack_frame: InterruptStackFrame) {
    serial_println!("Exception - Double Fault\n{:#?}", stack_frame);
}
