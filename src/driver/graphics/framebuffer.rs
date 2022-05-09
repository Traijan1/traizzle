pub fn draw_rectangle(buffer: &mut [u8], stride: usize, width: usize, height: usize, color: u32, offset_x: usize, offset_y: usize) {
    for y in 0..height {
        for x in (0..width * 4).step_by(4) {
            buffer[(x + offset_x + 0 + ((y + offset_y) * stride * 4)) as usize] = (color & 255) as u8;
            buffer[(x + offset_x + 1 + ((y + offset_y) * stride * 4)) as usize] = ((color >> 8u32) & 255) as u8;
            buffer[(x + offset_x + 2 + ((y + offset_y) * stride * 4)) as usize] = ((color >> 16u32) & 255) as u8;
            buffer[(x + offset_x + 3 + ((y + offset_y) * stride * 4)) as usize] = ((color >> 24u32) & 255) as u8;
        }
    }
}