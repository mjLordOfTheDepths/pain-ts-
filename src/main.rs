use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Icon};
use image::GenericImageView;
use std::thread;
mod taskbar; use crate::taskbar::{draw_taskbar, save_framebuffer_as_png};
mod constants; use crate::constants::*;
mod paint; use crate::paint::paint;
mod sound_track; use crate::sound_track::sound_track;

fn main() -> Result<(), Error> {
    // Load the icon image
    let icon_image = image::open("assets/ts.png").expect("Failed to open icon image");
    let (icon_width, icon_height) = icon_image.dimensions();
    let icon_rgba = icon_image.into_rgba8();
    let icon = Icon::from_rgba(icon_rgba.into_raw(), icon_width, icon_height).expect("Failed to create icon");

    // Window setup
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pain-ts-")
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT + TASKBAR_HEIGHT))
        .with_window_icon(Some(icon))
        .build(&event_loop)
        .unwrap();

    let mut size = window.inner_size();
    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(size.width, size.height, surface_texture)?;

    // Initialize the frame
    let mut framebuffer = vec![255; (size.width * size.height * 4) as usize]; 

    // Cursor and brush information
    let mut last_cursor_position = (0, 0);
    let mut left_button_pressed = false;
    let mut brush_size_modifier = 1;
    let mut current_colour = COLOURS[0]; // Red

    thread::spawn(|| sound_track()); // Spawning the sound track thread

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event { // Monitoring activity
            Event::WindowEvent { event, .. } => 
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    WindowEvent::Resized(new_size) => {resize(&mut pixels, &mut framebuffer, &mut size, new_size);}

                    WindowEvent::MouseInput { state, button, .. } => {handle_mouse_input(state, button, &mut left_button_pressed, last_cursor_position, &mut current_colour, &mut framebuffer, brush_size_modifier, size.width, size.height, &window);}
                    
                    WindowEvent::CursorMoved { position, .. } => {move_cursor(position, &mut last_cursor_position, left_button_pressed, &mut framebuffer, brush_size_modifier, size.width, size.height, current_colour, &window);}
                    
                    WindowEvent::MouseWheel { delta, .. } => {if let MouseScrollDelta::LineDelta(_, y) = delta {scroll_wheel(delta, &mut brush_size_modifier);}}
                    
                    _ => (),
            },
            
            Event::RedrawRequested(_) => { // Resizing taskbar
                draw_taskbar(&mut framebuffer, size.width, size.height);

                pixels.frame_mut().copy_from_slice(&framebuffer);
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}

// Mouse Input Handling :p

fn resize(pixels: &mut Pixels, framebuffer: &mut Vec<u8>, size: &mut PhysicalSize<u32>, new_size: PhysicalSize<u32>) {
    pixels.resize_surface(new_size.width, new_size.height).expect("Failed to resize surface");
    pixels.resize_buffer(new_size.width, new_size.height).expect("Failed to resize buffer");
    *framebuffer = vec![255; (new_size.width * new_size.height * 4) as usize];
    size.height = new_size.height;
    size.width = new_size.width;
}

fn handle_mouse_input(state: ElementState, button: MouseButton, left_button_pressed: &mut bool, last_cursor_position: (i32, i32), current_colour: &mut Colour, framebuffer: &mut Vec<u8>, brush_size_modifier: i32, width: u32, height: u32, window: &winit::window::Window) {
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
                    // Clicked on the white square
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

fn move_cursor(position: winit::dpi::PhysicalPosition<f64>, last_cursor_position: &mut (i32, i32), left_button_pressed: bool, framebuffer: &mut Vec<u8>, brush_size_modifier: i32, width: u32, height: u32, current_colour: Colour, window: &winit::window::Window) {
    let x = position.x as i32;
    let y = position.y as i32;
    *last_cursor_position = (x, y);
    if left_button_pressed {
        paint(framebuffer, x, y, brush_size_modifier, width, height - TASKBAR_HEIGHT, current_colour);
        window.request_redraw();
    }
}

fn scroll_wheel(delta: MouseScrollDelta, brush_size_modifier: &mut i32) {
    if let MouseScrollDelta::LineDelta(_, y) = delta {
        if y > 0.0 {
            *brush_size_modifier += 1;
        } else if y < 0.0 {
            *brush_size_modifier -= 1;
        }
    }
}