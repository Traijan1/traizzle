use super::fonts::psf::PSF; 

use crate::log;

pub struct Framebuffer<'a> {
    buffer: &'a mut [u8],
    stride: usize,
    channels: usize,
    font: PSF<'a>
}

impl<'a> Framebuffer<'a> {
    pub fn new(buffer: &'a mut [u8], stride: usize, channels: usize, font: PSF<'a>) -> Self {
        Self {
            buffer,
            stride,
            channels,
            font
        }
    } 

    pub fn draw_rectangle(&mut self, width: usize, height: usize, color: u32, offset_x: usize, offset_y: usize) {
        for y in 0..height {
            for x in (0..width * self.channels).step_by(self.channels) {
                self.buffer[(x + offset_x + 0 + ((y + offset_y) * self.stride * self.channels)) as usize] = (color & 255) as u8;
                self.buffer[(x + offset_x + 1 + ((y + offset_y) * self.stride * self.channels)) as usize] = ((color >> 8u32) & 255) as u8;
                self.buffer[(x + offset_x + 2 + ((y + offset_y) * self.stride * self.channels)) as usize] = ((color >> 16u32) & 255) as u8;
                self.buffer[(x + offset_x + 3 + ((y + offset_y) * self.stride * self.channels)) as usize] = ((color >> 24u32) & 255) as u8;
            }
        }
    }

    pub fn draw_char(&mut self, char: char, x: usize, y: usize) {
        let font_bytes = self.font.get_char(char);

        for (index, byte) in font_bytes.iter().enumerate() {
            log!("{:b}", *byte);
            for i in 0..8 {
                if *byte & (1 << i) != 0 {
                    log!("Test");
                    self.buffer[(x + i * self.channels + 0) + (y + index) * self.stride * self.channels] = 255;
                    self.buffer[(x + i * self.channels + 1) + (y + index) * self.stride * self.channels] = 255;
                    self.buffer[(x + i * self.channels + 2) + (y + index) * self.stride * self.channels] = 255;
                    self.buffer[(x + i * self.channels + 3) + (y + index) * self.stride * self.channels] = 0;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = 0;
        }
    }
}