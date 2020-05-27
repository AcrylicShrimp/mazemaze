extern crate sdl2;

use super::map::Map;
use super::map_renderer::MapRenderer;

pub struct World {
    map: Option<Map>,
    map_renderer: MapRenderer,
}

impl World {
    pub fn new(
        font: &sdl2::ttf::Font<'_, '_>,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> World {
        let mut map_renderer = MapRenderer::new(&font);

        map_renderer.register_block(0, ' ', sdl2::pixels::Color::WHITE, font, texture_creator);
        map_renderer.register_block(
            1,
            'W',
            sdl2::pixels::Color::from((96, 72, 96)),
            font,
            texture_creator,
        );
        map_renderer.register_block(
            2,
            'W',
            sdl2::pixels::Color::from((32, 32, 64)),
            font,
            texture_creator,
        );
        map_renderer.register_block(
            3,
            'W',
            sdl2::pixels::Color::from((128, 128, 96)),
            font,
            texture_creator,
        );

        World {
            map: None,
            map_renderer,
        }
    }

    pub fn map(&self) -> &Option<Map> {
        &self.map
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        match &self.map {
            Some(map) => self.map_renderer.render(canvas, &map),
            None => {}
        }
    }

    pub fn init_map(&mut self, map: Map) {
        self.map = Some(map);
    }
}
