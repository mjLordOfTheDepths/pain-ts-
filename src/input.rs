use pixels::Pixels;
use winit::dpi::PhysicalSize;
use winit::event::ElementState;
use winit::event::MouseButton;
use winit::event::MouseScrollDelta;
use crate::constants::*;
use crate::paint;
use crate::taskbar::save_framebuffer_as_png;

// input.rs
// This is a module for all functions that handle user input

pub fn resize(pixels: &mut Pixels, framebuffer: &mut Vec<u8>, size: &mut PhysicalSize<u32>, new_size: PhysicalSize<u32>) {
    pixels.resize_surface(new_size.width, new_size.height).expect("Failed to resize surface");
    pixels.resize_buffer(new_size.width, new_size.height).expect("Failed to resize buffer");
    *framebuffer = vec![255; (new_size.width * new_size.height * 4) as usize];
    size.height = new_size.height;
    size.width = new_size.width;
}

pub fn handle_mouse_input(state: ElementState, button: MouseButton, left_button_pressed: &mut bool, last_cursor_position: (i32, i32), current_colour: &mut Colour, framebuffer: &mut Vec<u8>, brush_size_modifier: i32, width: u32, height: u32, window: &winit::window::Window) {
    if button == MouseButton::Left {
        if state == ElementState::Pressed {
            *left_button_pressed = true;
            let (x, y) = last_cursor_position;
            let taskbar_start = height - TASKBAR_HEIGHT;
            if y >= taskbar_start as i32 && y < height as i32 {
                let button_index = (x / BUTTON_SIZE as i32) as usize;
                if button_index < COLOURS.len() {
                    *current_colour = COLOURS[button_index];
                } else if x >= (width - BUTTON_SIZE) as i32 {
                    // Clicked on the save icon
                    save_framebuffer_as_png(framebuffer, width, height);
                }
            } else {
                paint(framebuffer, x, y, brush_size_modifier, width, height - TASKBAR_HEIGHT, *current_colour);
                window.request_redraw();
            }
        } else {
            *left_button_pressed = false;
        }
    }
}

pub fn move_cursor(position: winit::dpi::PhysicalPosition<f64>, last_cursor_position: &mut (i32, i32), left_button_pressed: bool, framebuffer: &mut Vec<u8>, brush_size_modifier: i32, width: u32, height: u32, current_colour: Colour, window: &winit::window::Window) {
    let x = position.x as i32;
    let y = position.y as i32;
    *last_cursor_position = (x, y);
    if left_button_pressed {
        paint(framebuffer, x, y, brush_size_modifier, width, height - TASKBAR_HEIGHT, current_colour);
        window.request_redraw();
    }
}

pub fn scroll_wheel(delta: MouseScrollDelta, brush_size_modifier: &mut i32) {
    if let MouseScrollDelta::LineDelta(_, y) = delta {
        if y > 0.0 {
            *brush_size_modifier += 1;
        } else if y < 0.0 {
            *brush_size_modifier -= 1;
        }
    }
}