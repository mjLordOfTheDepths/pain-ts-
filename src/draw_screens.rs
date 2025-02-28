use image::imageops::FilterType;
use crate::constants::*;

// draw_screens.rs
// This module manages the menu the game modes

pub fn draw_initial_screen(framebuffer: &mut Vec<u8>, width: u32, height: u32) {
    let icon = &*ICON_IMAGE;
    let icon_width = (width as f32 * 0.2) as u32;
    let icon_height = (icon.height() as f32 * (icon_width as f32 / icon.width() as f32)) as u32;
    let icon_x = (width - icon_width) / 2;
    let icon_y = (height as f32 * 0.2) as u32 - icon_height / 2;

    // Resize the icon to fit the allocated area
    let resized_icon = image::imageops::resize(icon, icon_width, icon_height, FilterType::Lanczos3);

    let classic = &*CLASSIC_IMAGE;
    let terrain = &*TERRAIN_IMAGE;
    let rect_width = (width as f32 * 0.75) as u32;
    let rect_height = (height as f32 * 0.1) as u32;
    let classic_height = rect_height;
    let classic_width = (classic.width() as f32 * (classic_height as f32 / classic.height() as f32)) as u32;
    let terrain_height = rect_height;
    let terrain_width = (terrain.width() as f32 * (terrain_height as f32 / terrain.height() as f32)) as u32;
    let classic_x = (width - classic_width) / 2;
    let classic_y = (height - rect_height) / 2;

    // Position for the terrain image relative to the classic image
    let terrain_y = classic_y + rect_height + 25; // 25 pixels below the first classic image

    // Resize the classic and terrain images to fit the allocated area
    let resized_classic = image::imageops::resize(&*classic, classic_width, classic_height, FilterType::Lanczos3);
    let resized_terrain = image::imageops::resize(&*terrain, terrain_width, terrain_height, FilterType::Lanczos3);

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
            } else if x >= classic_x && x < classic_x + terrain_width && y >= terrain_y && y < terrain_y + terrain_height { // Rendering the terrain game mode icon
                let pixel = resized_terrain.get_pixel((x - classic_x) as u32, (y - terrain_y) as u32);
                framebuffer[offset] = pixel[0];
                framebuffer[offset + 1] = pixel[1];
                framebuffer[offset + 2] = pixel[2];
                framebuffer[offset + 3] = pixel[3];
            } else if (x >= (width - rect_width) / 2 && x <= (width + rect_width) / 2 && y >= (height - rect_height) / 2 && y <= (height + rect_height) / 2) || // Rendering a white border for classic
                      (x >= (width - rect_width) / 2 && x <= (width + rect_width) / 2 && y >= terrain_y && y <= terrain_y + rect_height) { // Rendering a white border for terrain
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