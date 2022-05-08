#[macro_export]
macro_rules! log {
    ($message:tt) => {
        serial_port::write_serial("LOG: ");
        serial_port::write_serial($message);
        serial_port::write_serial("\n");
    }
}