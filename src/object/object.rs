extern crate sdl2;

pub struct Object {
    pub x: i32,
    pub y: i32,
    glyph: char,
    color: sdl2::pixels::Color,
}

impl Object {
    pub fn new(x: i32, y: i32, glyph: char, color: sdl2::pixels::Color) -> Object {
        Object { x, y, glyph, color }
    }

    pub fn color(&self) -> sdl2::pixels::Color {
        self.color
    }
}
