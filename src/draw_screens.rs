use image::imageops::FilterType;
use crate::constants::*;

// draw_screens.rs
// This module manages the menu and entering the classic game mode

pub fn draw_initial_screen(framebuffer: &mut Vec<u8>, width: u32, height: u32) {
    let icon = &*ICON_IMAGE;
    let icon_width = (width as f32 * 0.2) as u32;
    let icon_height = (icon.height() as f32 * (icon_width as f32 / icon.width() as f32)) as u32;
    let icon_x = (width - icon_width) / 2;
    let icon_y = (height as f32 * 0.2) as u32 - icon_height / 2;

    // Resize the icon to fit the allocated area
    let resized_icon = image::imageops::resize(icon, icon_width, icon_height, FilterType::Lanczos3);

    let classic = &*CLASSIC_IMAGE;
    let rect_width = (width as f32 * 0.75) as u32;
    let rect_height = (height as f32 * 0.1) as u32;
    let classic_height = rect_height;
    let classic_width = (classic.width() as f32 * (classic_height as f32 / classic.height() as f32)) as u32;
    let classic_x = (width - classic_width) / 2;
    let classic_y = (height - rect_height) / 2;

    // Resize the classic image to fit the allocated area
    let resized_classic = image::imageops::resize(&*classic, classic_width, classic_height, FilterType::Lanczos3);

    for y in 0..height { // Drawing height
        for x in 0..width { // Drawing width
            let offset = ((y * width + x) * 4) as usize;

            if x >= icon_x && x < icon_x + icon_width && y >= icon_y && y < icon_y + icon_height { // Rendering the logo
                let pixel = resized_icon.get_pixel((x - icon_x) as u32, (y - icon_y) as u32);
                framebuffer[offset] = pixel[0];
                framebuffer[offset + 1] = pixel[1];
                framebuffer[offset + 2] = pixel[2];
                framebuffer[offset + 3] = pixel[3];
            } else if x >= classic_x && x < classic_x + classic_width && y >= classic_y && y < classic_y + classic_height { // Rendering the classic game mode icon
                let pixel = resized_classic.get_pixel((x - classic_x) as u32, (y - classic_y) as u32);
                framebuffer[offset] = pixel[0];
                framebuffer[offset + 1] = pixel[1];
                framebuffer[offset + 2] = pixel[2];
                framebuffer[offset + 3] = pixel[3];
            } else if x >= (width - rect_width) / 2 && x <= (width + rect_width) / 2 && y >= (height - rect_height) / 2 && y <= (height + rect_height) / 2 { // Rendering a white border for classic
                framebuffer[offset] = 255;     // Red
                framebuffer[offset + 1] = 255; // Green
                framebuffer[offset + 2] = 255; // Blue
                framebuffer[offset + 3] = 255; // Alpha
            } else {
                framebuffer[offset] = 200;     // Red
                framebuffer[offset + 1] = 200; // Green
                framebuffer[offset + 2] = 200; // Blue
                framebuffer[offset + 3] = 255; // Alpha
            }
        }
    }
}

pub fn draw_white_screen(framebuffer: &mut Vec<u8>, width: u32, height: u32) { // Replaces the menu with the canvas
    for y in 0..height {
        for x in 0..width {
            let offset = ((y * width + x) * 4) as usize;
            framebuffer[offset] = 255;     // Red
            framebuffer[offset + 1] = 255; // Green
            framebuffer[offset + 2] = 255; // Blue
            framebuffer[offset + 3] = 255; // Alpha
        }
    }
}