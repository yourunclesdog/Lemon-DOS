use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

//usage of lazy_static ensures that the init method is called exactly once
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        //the unsafe SerialPort::new expects the address of the first I/O port, from which it
        //calculates the addresses of the rest of the ports.
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
    //a spinlock is used to create a static writer instance.
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed.");
}

//prints to serial interface
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
       $crate::serial::_print(format_args!($($arg)*)); 
    };
}

//prints to serial interface but with println sort of print
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}