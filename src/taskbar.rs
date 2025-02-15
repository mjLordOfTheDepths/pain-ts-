use crate::constants::*;
pub fn draw_taskbar(framebuffer: &mut Vec<u8>, width: u32, height: u32) {
    let taskbar_start = height - TASKBAR_HEIGHT;
    for y in taskbar_start..height {
        for x in 0..width {
            let index = (y as usize * width as usize + x as usize) * 4;
            framebuffer[index] = 200;     // Red
            framebuffer[index + 1] = 200; // Green
            framebuffer[index + 2] = 200; // Blue
            framebuffer[index + 3] = 255; // Alpha
        }
    }


    for (i, colour) in COLOURS.iter().enumerate() {
        let x_start = (i as u32 * BUTTON_SIZE) as usize;
        let x_end = x_start + BUTTON_SIZE as usize;
        let y_start = (taskbar_start + TASKBAR_HEIGHT - BUTTON_SIZE) as usize;
        let y_end = (taskbar_start + TASKBAR_HEIGHT) as usize;

        for y in y_start..y_end {
            for x in x_start..x_end {
                let index = (y * width as usize + x) * 4;
                framebuffer[index] = colour.r;
                framebuffer[index + 1] = colour.g;
                framebuffer[index + 2] = colour.b;
                framebuffer[index + 3] = 255;
            }
        }
    }
}