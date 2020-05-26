extern crate sdl2;

pub struct Renderer {
    width: u32,
    height: u32,
    texture: sdl2::render::Texture,
}

impl Renderer {
    pub fn new(
        glyph: char,
        color: sdl2::pixels::Color,
        font: &sdl2::ttf::Font<'_, '_>,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Renderer {
        let size = font.size_of_char(glyph).unwrap();

        Renderer {
            width: size.0,
            height: size.1,
            texture: texture_creator
                .create_texture_from_surface(&font.render_char(glyph).blended(color).unwrap())
                .unwrap(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
        canvas
            .copy(
                &self.texture,
                None,
                sdl2::rect::Rect::new(
                    x * (self.width + 4) as i32,
                    y * self.height as i32,
                    self.width,
                    self.height,
                ),
            )
            .unwrap();
    }
}
