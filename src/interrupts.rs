use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::serial_println;
use lazy_static::lazy_static;

//usage of lazy_static ensures that this is only called once
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(doublefault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

//displays a message upon breakpoint interrupts
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    serial_println!("Exception - Breakpoint\n{:#?}", stack_frame);
}

//panics on double fault
extern "x86-interrupt" fn doublefault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Exception - Double Fault\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    serial_println!("Exception - Page Fault");
    serial_println!("At: {:?}", Cr2::read());
    serial_println!("Error Code: {:?}", error_code);
    serial_println!("{:#?}", stack_frame);
    panic!();
}
