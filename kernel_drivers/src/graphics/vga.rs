use crate::driver::{GraphicsWriter, Writer};

const VGA_BUFFER_ADDRESS: usize = 0xb8000;
const BUFFER_HEIGHT: usize = 10;
const BUFFER_WIDTH: usize = 20;

// Represents a character and its color attribute
#[derive(Clone, Copy)]
#[repr(C)] // Ensure the struct has the same memory layout as in C
struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

// The VGA text buffer is a 2D array of ScreenChar
type VgaBufferArray = [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT];

// Mache VgaBuffer öffentlich
pub struct VgaBuffer {
    column_position: usize,
    row_position: usize,
    color_code: u8, // Default color
}

impl VgaBuffer {
    // Ein öffentlicher Konstruktor
    pub fn new() -> Self {
        VgaBuffer {
            column_position: 0,
            row_position: 0,
            color_code: 0x0f, // Default: White on Black
        }
    }

    // Writes a single byte to the buffer at the current position
    // Handles newlines, wrapping, and scrolling
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                let vga_buffer = VGA_BUFFER_ADDRESS as *mut VgaBufferArray;

                unsafe {
                    (*vga_buffer)[row][col] = ScreenChar {
                        ascii_character: byte,
                        color_code: self.color_code,
                    };
                }

                self.column_position += 1;
            }
        }
    }
}

impl Writer for VgaBuffer {
    fn write(&mut self, buffer: &[u8]) -> usize {
        for &byte in buffer {
            self.write_byte(byte);
        }
        buffer.len()
    }
}

impl GraphicsWriter for VgaBuffer {
    fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.row_position = 0;
        self.column_position = 0;
    }

    fn set_color(&mut self, color_code: u8) {
        self.color_code = color_code;
    }

    fn new_line(&mut self) {
        self.row_position += 1;
        self.column_position = 0;

        if self.row_position >= BUFFER_HEIGHT {
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        let vga_buffer = VGA_BUFFER_ADDRESS as *mut VgaBufferArray;

        // Shift all rows up by one
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                unsafe {
                    (*vga_buffer)[row - 1][col] = (*vga_buffer)[row][col];
                }
            }
        }

        // Clear the last row
        self.clear_row(BUFFER_HEIGHT - 1);

        // Move the cursor to the beginning of the last row
        self.row_position = BUFFER_HEIGHT - 1;
        self.column_position = 0;
    }

    // Clears a specific row by writing spaces with the current color
    fn clear_row(&mut self, row: usize) {
        let blank_char = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        let vga_buffer = VGA_BUFFER_ADDRESS as *mut VgaBufferArray;

        for col in 0..BUFFER_WIDTH {
            unsafe {
                (*vga_buffer)[row][col] = blank_char;
            }
        }
    }
}
