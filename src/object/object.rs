extern crate sdl2;

use super::renderer::Renderer;

pub struct Object {
    pub x: i32,
    pub y: i32,
    renderer: Renderer,
}

impl Object {
    pub fn new(
        x: i32,
        y: i32,
        glyph: char,
        color: sdl2::pixels::Color,
        font: &sdl2::ttf::Font<'_, '_>,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Object {
        Object {
            x,
            y,
            renderer: Renderer::new(glyph, color, &font, &texture_creator),
        }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.renderer.render(canvas, self.x, self.y);
    }
}
