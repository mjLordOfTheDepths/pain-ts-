use once_cell::sync::Lazy;

// constants.rs
//This module contains all constants

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const TASKBAR_HEIGHT: u32 = 75;
pub const BUTTON_SIZE: u32 = 40;

pub static ICON_IMAGE: Lazy<image::RgbaImage> = Lazy::new(|| {
    image::open("assets/ts.png").expect("Failed to open icon image").into_rgba8()
});

pub static CLASSIC_IMAGE: Lazy<image::RgbaImage> = Lazy::new(|| {
    image::open("assets/classic.png").expect("Failed to open classic image").into_rgba8()
});

#[derive(Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const COLOURS: [Colour; 14] = [
    Colour { r: 255, g: 0, b: 0 },    // Richard
    Colour { r: 255, g: 165, b: 0 },  // Of
    Colour { r: 255, g: 255, b: 0 },  // York
    Colour { r: 0, g: 255, b: 0 },    // Gave
    Colour { r: 0, g: 0, b: 255 },    // Battle
    Colour { r: 75, g: 0, b: 130 },   // In
    Colour { r: 238, g: 130, b: 238 },// Vain
    Colour { r: 255, g: 255, b: 255 }, // Wow
    Colour { r: 0, g: 0, b: 0 },       // Black
    Colour { r: 84, g: 255, b: 255 }, // Light blue
    Colour { r: 201, g: 201, b: 201 }, // Grey
    Colour { r: 176, g: 94, b: 4 }, // Brown
    Colour { r: 150, g: 149, b: 54 }, // Dark Green 
    Colour { r: 255, g: 0, b: 255 }, // Transparent / Magenta
];