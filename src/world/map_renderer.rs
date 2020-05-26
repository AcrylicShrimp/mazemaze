extern crate sdl2;

use super::map::Map;

pub struct MapRenderer {
    block_map: std::collections::HashMap<u8, (sdl2::render::Texture, u32, u32)>,
    line_height: u32,
}

impl MapRenderer {
    pub fn new(font: &sdl2::ttf::Font<'_, '_>) -> MapRenderer {
        MapRenderer {
            block_map: std::collections::HashMap::new(),
            line_height: font.height() as u32,
        }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, map: &Map) {
        let mut pos_x: u32 = 0;
        let mut pos_y: u32 = 0;

        for y in 0..map.height() {
            for x in 0..map.width() {
                match self.block_map.get(&map.get_block(x, y)) {
                    Some(block) => {
                        canvas
                            .copy(
                                &block.0,
                                None,
                                sdl2::rect::Rect::new(pos_x as i32, pos_y as i32, block.1, block.2),
                            )
                            .unwrap();
                        pos_x += block.1 + 4;
                    }
                    None => (),
                }
            }

            pos_x = 0;
            pos_y += self.line_height;
        }
    }

    pub fn register_block(
        &mut self,
        id: u8,
        glyph: char,
        color: sdl2::pixels::Color,
        font: &sdl2::ttf::Font<'_, '_>,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) {
        self.block_map.insert(
            id,
            (
                texture_creator
                    .create_texture_from_surface(&font.render_char(glyph).blended(color).unwrap())
                    .unwrap(),
                font.size_of_char(glyph).unwrap().0,
                font.size_of_char(glyph).unwrap().1,
            ),
        );
    }
}
