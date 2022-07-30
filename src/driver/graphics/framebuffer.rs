use super::fonts::psf::PSF; 

pub struct Framebuffer<'a> {
    buffer: &'a mut [u8],
    stride: usize,
    pub channels: usize,
    pub font: PSF<'a>,
    pub foreground: u32,
    pub background: u32,
}

impl<'a> Framebuffer<'a> {
    const WHITE: u32 = 0x00FFFFFF;
    const BLACK: u32 = 0x00000000;

    pub fn new(buffer: &'a mut [u8], stride: usize, channels: usize, font: PSF<'a>) -> Self {
        Self {
            buffer,
            stride,
            channels,
            font,
            foreground: Self::WHITE,
            background: Self::BLACK,
        }
    } 

    pub fn draw_rectangle(&mut self, width: usize, height: usize, color: u32, offset_x: usize, offset_y: usize) {
        for y in 0..height {
            for x in (0..width * self.channels).step_by(self.channels) {
                let pixel = (x + offset_x + ((y + offset_y) * self.stride * self.channels)) as usize;
                self.draw_pixel(pixel, color);
            }
        }
    }

    pub fn print_char(&mut self, char: char, x: usize, y: usize) {
        let font_bytes = self.font.get_char(char);

        for (index, byte) in font_bytes.iter().enumerate() {
            for i in 0..8 {
                let pixel = (x + (PSF::CHAR_WIDTH - i) * self.channels) + (y * PSF::CHAR_HEIGHT + index) * self.stride * self.channels;
                
                if *byte & (1 << i) != 0 {
                    self.draw_pixel(pixel, self.foreground);
                }
                else {
                    self.draw_pixel(pixel, self.background);
                }
            }
        }
    }

    #[inline]
    fn draw_pixel(&mut self, pos: usize, color: u32) {
        self.buffer[pos] = (color & 255) as u8;
        self.buffer[pos + 1] = ((color >> 8) & 255) as u8;
        self.buffer[pos + 2] = ((color >> 16) & 255) as u8;
        self.buffer[pos + 3] = ((color >> 24) & 255) as u8;
    }

    pub fn clear(&mut self) {
        for i in (0..self.buffer.len()).step_by(4) {
            self.draw_pixel(i, self.background);
        }
    }
}