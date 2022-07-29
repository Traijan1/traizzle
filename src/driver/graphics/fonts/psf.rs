pub struct PSF<'a> {
    pub magic: &'a [u8],
    pub file_mode: u8,
    pub font_height: u8,
    pub font_data: &'a [u8],
    pub is_psf_font: bool
}

impl<'a> PSF<'a> {
    const MAGIC1: u8 = 0x36;
    const MAGIC2: u8 = 0x04;
    pub const CHAR_WIDTH: usize = 8;
    pub const CHAR_HEIGHT: usize = 16;

    pub fn new(data: &'a [u8]) -> Self {
        let mut font = Self {
            magic: &data[0..2],
            file_mode: data[2],
            font_height: data[3],
            font_data: &data[4..data.len()],
            is_psf_font: false
        };

        font.is_psf_font = font.magic[0] == Self::MAGIC1 && font.magic[1] == Self::MAGIC2;

        font
    }

    pub fn get_char(&self, char: char) -> &'a [u8] {
        &self.font_data[char as usize * self.font_height as usize..][..self.font_height as usize]
    }
}