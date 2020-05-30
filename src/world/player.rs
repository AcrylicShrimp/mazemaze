extern crate sdl2;

use sdl2::pixels;

pub struct Player {
    id: u64,
    pub x: i32,
    pub y: i32,
    color: pixels::Color,
}

impl Player {
    pub fn new(id: u64, x: i32, y: i32, color: pixels::Color) -> Player {
        Player { id, x, y, color }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn color(&self) -> pixels::Color {
        self.color
    }
}
