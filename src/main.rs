use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Icon};
use image::GenericImageView;
use std::thread;
mod taskbar; use crate::taskbar::{draw_taskbar, terrain_taskbar};
mod constants; use crate::constants::*;
mod paint; 
mod sound_track; 
use crate::sound_track::sound_track;
mod draw_screens; use crate::draw_screens::{draw_initial_screen, draw_white_screen};
mod input; use crate::input::{resize, handle_mouse_input, move_cursor, scroll_wheel};

enum AppState {
    InitialScreen,
    MainLoop,
    TerrainMode,
}

// main.rs
// This is the main file that runs the program
// It also handles window setup and some minor event handling that didn't consitute a separate function

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
    let mut current_colour = COLOURS[7]; // White

    let mut app_state = AppState::InitialScreen;
    let mut save_directory = "../my_pics"; // default screenshots

    thread::spawn(|| sound_track()); // Spawning the sound track thread

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    resize(&mut pixels, &mut framebuffer, &mut size, new_size);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if let AppState::InitialScreen = app_state {
                        if button == MouseButton::Left && state == ElementState::Pressed {
                            let (x, y) = last_cursor_position;
                            let rect_width = (size.width as f32 * 0.75) as i32;
                            let rect_height = (size.height as f32 * 0.15) as i32;
                            let rect_x = (size.width as i32 - rect_width) / 2;
                            let rect_y = (size.height as i32 - rect_height) / 2;
                            let terrain_y = rect_y + rect_height + 25; // 25 pixels below the first classic image
                            if x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height {
                                app_state = AppState::MainLoop;
                                draw_white_screen(&mut framebuffer, size.width, size.height);
                            } else if x >= rect_x && x <= rect_x + rect_width && y >= terrain_y && y <= terrain_y + rect_height {
                                app_state = AppState::TerrainMode;
                                draw_white_screen(&mut framebuffer, size.width, size.height);
                                terrain_taskbar(&mut framebuffer, size.width, size.height);
                            }
                        }
                    } else {
                        let colours = if let AppState::TerrainMode = app_state {
                            save_directory = "../terrain";
                            &[
                                COLOURS[1],  // Of
                                COLOURS[3],  // Gave
                                COLOURS[4],  // Battle
                                COLOURS[7],  // Wow
                                COLOURS[9],  // Light blue
                                COLOURS[10], // Grey
                                COLOURS[11], // Brown
                                COLOURS[12], // Dark Green
                            ]
                        } else {
                            //save_directory = "../my_pics";
                            &COLOURS[..7]
                        };

                        handle_mouse_input(
                            state,
                            button,
                            &mut left_button_pressed,
                            last_cursor_position,
                            &mut current_colour,
                            &mut framebuffer,
                            brush_size_modifier,
                            size.width,
                            size.height,
                            &window,
                            colours,
                            save_directory,
                        );
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    last_cursor_position = (position.x as i32, position.y as i32);
                    if let AppState::MainLoop = app_state {
                        move_cursor(
                            position,
                            &mut last_cursor_position,
                            left_button_pressed,
                            &mut framebuffer,
                            brush_size_modifier,
                            size.width,
                            size.height,
                            current_colour,
                            &window,
                        );
                    } else if let AppState::TerrainMode = app_state {
                        move_cursor(
                            position,
                            &mut last_cursor_position,
                            left_button_pressed,
                            &mut framebuffer,
                            brush_size_modifier,
                            size.width,
                            size.height,
                            current_colour,
                            &window,
                        );
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    if let AppState::MainLoop = app_state {
                        scroll_wheel(delta, &mut brush_size_modifier);
                    } else if let AppState::TerrainMode = app_state {
                        scroll_wheel(delta, &mut brush_size_modifier);
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                if let AppState::InitialScreen = app_state {
                    draw_initial_screen(&mut framebuffer, size.width, size.height);
                } else if let AppState::TerrainMode = app_state {
                    terrain_taskbar(&mut framebuffer, size.width, size.height);
                } else {
                    draw_taskbar(&mut framebuffer, size.width, size.height);
                }

                pixels.frame_mut().copy_from_slice(&framebuffer);
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}

