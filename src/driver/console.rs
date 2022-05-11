use super::graphics::framebuffer::Framebuffer;
use core::fmt;

pub struct Console<'a> {
    framebuffer: &'a mut Framebuffer<'a>,
    column: usize
}

impl<'a> Console<'a> {
    pub fn new(framebuffer: &'a mut Framebuffer<'a>) -> Self {
        Self { 
            framebuffer,
            column: 0
        }
    }

    pub fn write(&mut self, text: &str) {
        text.chars().for_each(|char| {
            self.framebuffer.print_char(char, self.column * 8 * self.framebuffer.channels, 0);
            self.column += 1;
        });
    }
}

impl<'a> fmt::Write for Console<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);        
        Ok(())
    }
}