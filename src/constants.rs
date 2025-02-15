pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const TASKBAR_HEIGHT: u32 = 75;
pub const BUTTON_SIZE: u32 = 40;

#[derive(Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const COLOURS: [Colour; 8] = [
    Colour { r: 255, g: 0, b: 0 },    // Richard
    Colour { r: 255, g: 165, b: 0 },  // Of
    Colour { r: 255, g: 255, b: 0 },  // York
    Colour { r: 0, g: 255, b: 0 },    // Gave
    Colour { r: 0, g: 0, b: 255 },    // Battle
    Colour { r: 75, g: 0, b: 130 },   // In
    Colour { r: 238, g: 130, b: 238 },// Vain
    Colour { r: 255, g: 255, b: 255 } // Wow
];