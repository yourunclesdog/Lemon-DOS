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
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

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

    //playing around with writing to memory
    //let foo = 0x0000_1100 as *mut u32;
    //unsafe { 
    //    *foo = 6920; 
    //    *foo = *foo + 20;
    //}
    //serial_println!("{}", unsafe{*foo});

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