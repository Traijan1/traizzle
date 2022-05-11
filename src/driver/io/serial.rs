use core::fmt;
use crate::driver::io::port::Port;

/// The location of COM1
pub const COM1: u16 = 0x3F8;

/// Represents a Serial Port
pub struct Serial {
    port: Port
}

impl Serial {
    /// Creates the Serial Struct and initializes the port
    pub fn new(com: u16) -> Self {
        let serial = Self {
            port: Port::new(com)
        };

        serial.initialize();

        serial
    }

    pub fn write_char(&self, char: char) {
        // loop {
        //     if Serial::is_transmit_empty(&self.port) == 0 {
        //         break;
        //     }
        // }

        self.port.write_u8(char as u8);
    }

    pub fn write(&self, message: &str) {
        message.chars().for_each(|char| self.write_char(char));
    }

    fn is_transmit_empty(port: &Port) -> u8 {
        port.read_u8_offset(5) & 0x20
    }

    fn initialize(&self) {
        self.port.write_u8_offset(1, 0x00); // Disable interrupts for this port
        self.port.write_u8_offset(3, 0x80); // Enable DLAB => set baud rate divisor
        self.port.write_u8(0x03); // Set divisor to 3 (lo byte) which is 38400 baud
        self.port.write_u8_offset(1, 0x00); // hi byte
        self.port.write_u8_offset(3, 0x03); // 8 bits, no parity, one stop bit
        self.port.write_u8_offset(2, 0xC7); // Enable FIFO, cler them, with 14-byte threshold
        self.port.write_u8_offset(4, 0x0B); // IRQs enabled, RTS/DSR set

        // Testing port
        self.port.write_u8_offset(4, 0x1E); // Set in loopback mode
        self.port.write_u8(0xAE);

        let is_faulty = self.port.read_u8() != 0xAE;
        // Do smth when faulty

        // Set to normal mode
        self.port.write_u8_offset(4, 0x0F);
    } 
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}