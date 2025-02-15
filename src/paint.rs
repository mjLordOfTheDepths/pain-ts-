use crate::constants::*;

pub fn paint(framebuffer: &mut Vec<u8>, x: i32, y: i32, modifier: i32, width: u32, height: u32, colour: Colour) {
    let brush_size = 5 * modifier;
    for dy in 0..brush_size {
        for dx in 0..brush_size {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let index = (ny as usize * width as usize + nx as usize) * 4;
                framebuffer[index] = colour.r;     // Red
                framebuffer[index + 1] = colour.g; // Green
                framebuffer[index + 2] = colour.b; // Blue
                framebuffer[index + 3] = 255;     // Alpha
            }
        }
    }
}

