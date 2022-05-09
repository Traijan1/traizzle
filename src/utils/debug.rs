use crate::driver::io::serial::{Serial, COM1};
use lazy_static::lazy_static;

use spin::Mutex;

use core::fmt;

lazy_static! {
    pub static ref SERIAL_COM1: Mutex<Serial> = {
        Mutex::new(Serial::new(COM1))
    };
}

/// Writes in COM1 to write debug messages to qemu -serial arg
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        crate::utils::debug::SERIAL_COM1.lock().write("LOG: ");
        $crate::debug::_log(format_args!($($arg)*));
        crate::utils::debug::SERIAL_COM1.lock().write("\n");
    }
}

pub fn _log(args: fmt::Arguments) {
    use core::fmt::Write;
    crate::debug::SERIAL_COM1.lock().write_fmt(args).unwrap();
}


