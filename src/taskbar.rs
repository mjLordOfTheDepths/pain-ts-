use crate::constants::*;
use image::{ImageBuffer, Rgba};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use chrono::Local;

// Load the save icon image once and store it in a static variable
static SAVE_ICON: Lazy<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>> = Lazy::new(|| {
    let save_icon = image::open("assets/save.png").expect("Failed to open save icon image");
    let save_icon = save_icon.resize_exact(BUTTON_SIZE, BUTTON_SIZE, image::imageops::FilterType::Lanczos3);
    Mutex::new(save_icon.to_rgba8())
});

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

    // Draw the save icon on the right side of the taskbar
    let square_size = BUTTON_SIZE as usize;
    let x_start = (width as usize - square_size) as usize;
    let y_start = (taskbar_start + TASKBAR_HEIGHT - square_size as u32) as usize;

    let save_icon = SAVE_ICON.lock().unwrap();
    let save_icon_pixels = save_icon.as_flat_samples().samples;
    let save_icon_width = save_icon.width() as usize;
    let save_icon_height = save_icon.height() as usize;

    for y in 0..save_icon_height {
        let src_index = y * save_icon_width * 4;
        let dst_index = ((y_start as usize + y) * width as usize + x_start) * 4;
        framebuffer[dst_index..dst_index + save_icon_width * 4].copy_from_slice(&save_icon_pixels[src_index..src_index + save_icon_width * 4]);
    }
}

pub fn save_framebuffer_as_png(framebuffer: &Vec<u8>, width: u32, height: u32) {
    let taskbar_height = TASKBAR_HEIGHT as usize;
    let new_height = height as usize - taskbar_height;
    let new_width = width as usize;

    let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(new_width as u32, new_height as u32);

    for y in 0..new_height {
        for x in 0..new_width {
            let src_index = (y * width as usize + x) * 4;
            let pixel = Rgba([
                framebuffer[src_index],
                framebuffer[src_index + 1],
                framebuffer[src_index + 2],
                framebuffer[src_index + 3],
            ]);
            buffer.put_pixel(x as u32, y as u32, pixel);
        }
    }

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let file_path = format!("../my_pics/{}.png", timestamp);
    buffer.save(file_path).unwrap();
}