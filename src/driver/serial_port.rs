const COM1: u16 = 0x3F8;

use crate::driver::io::port::commands::{inb, outb};

/// Initializes the serial port COM1
/// 
/// Returns 1 if port is faulty and 0 if not
pub fn initialize_serial() -> u8 {
    outb(COM1 + 1, 0x00);    // Disable all interrupts
    outb(COM1 + 3, 0x80);    // Enable DLAB (set baud rate divisor)
    outb(COM1 + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
    outb(COM1 + 1, 0x00);    //                  (hi byte)
    outb(COM1 + 3, 0x03);    // 8 bits, no parity, one stop bit
    outb(COM1 + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
    outb(COM1 + 4, 0x0B);    // IRQs enabled, RTS/DSR set
    outb(COM1 + 4, 0x1E);    // Set in loopback mode, test the serial chip
    outb(COM1 + 0, 0xAE);    // Test serial chip (send byte 0xAE and check if serial returns same byte)

    // Check if faulty
    if inb(COM1) != 0xAE {
        return 1;
    };

    outb(COM1 + 4, 0x0F);
    
    0
}

fn is_transmit_empty() -> u8 {
    inb(COM1 + 5) & 0x20
}

pub fn write_serial_char(a: char) {
    // 'check: loop {
    //     if is_transmit_empty() == 0 {
    //         break 'check;
    //     }
    // }

    outb(COM1, a as u8);
}

pub fn write_serial(a: &str) {
    a.chars().for_each(|char| write_serial_char(char));
}