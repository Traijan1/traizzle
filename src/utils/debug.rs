use crate::driver::io::serial::{Serial, COM1};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL_COM1: Serial = {
        Serial::new(COM1)
    };
}

#[macro_export]
macro_rules! log {
    ($message:tt) => {
        debug::SERIAL_COM1.write_str("LOG: ");
        debug::SERIAL_COM1.write_str($message);
        debug::SERIAL_COM1.write_str("\n");
    }
}