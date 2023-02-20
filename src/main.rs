#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod serial;
mod interrupts;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    serial_print!("hello world");
    x86_64::instructions::interrupts::int3();
    loop{}
}

fn init() {
    interrupts::init_idt();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}