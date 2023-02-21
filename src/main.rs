//the std library cannot be used as there is no OS to provide it (duh)
#![no_std]
//a c _start is required due to the lack of a rust runtime; this replaces the main function
#![no_main]
//interrupts
#![feature(abi_x86_interrupt)]

mod serial;
mod interrupts;

use core::panic::PanicInfo;
use x86_64::instructions::hlt;

//disable name mangling, as the os needs to start with a C _start
#[no_mangle]
pub extern "C" fn _start() -> ! {

    //initialise the os, for now only the idt
    init();

    //print "hello world" to serial terminal
    serial_println!("hello world");

    //to trigger a breakpoint interrupt
    //x86_64::instructions::interrupts::int3();

    //write to an invalid address in order to trigger a double fault
    //unsafe {
    //    *(0xdeadbeef as *mut u64) = 42;
    //};

    //halt loop forever
    loop{
        hlt();
    }
}

fn init() {
    //initialise idt
    interrupts::init_idt();
}

//what to do if the os panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{
        hlt();
    }
}