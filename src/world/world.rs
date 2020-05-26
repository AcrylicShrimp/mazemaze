extern crate sdl2;

use super::gen::Gen;
use super::map::Map;
use super::map_generator::generate_map;
use super::map_renderer::MapRenderer;

pub struct World {
    map: Map,
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

        let mut gen = Gen::new();

        gen.add_tile(0, 2f32);
        gen.add_tile(1, 4f32);
        gen.add_tile(2, 4f32);
        gen.add_tile(3, 1f32);

        let mut constraints = std::collections::HashMap::new();

        constraints.insert(0, vec![0, 3]);
        constraints.insert(1, vec![1, 2, 3]);
        constraints.insert(2, vec![1, 2, 3]);
        constraints.insert(3, vec![0, 1, 2, 3]);

        let mut initials: Vec<Vec<bool>> = Vec::new();

        for _ in 0..40 * 30 {
            initials.push(vec![true; 4]);
        }

        for index in 0..40 * 30 {
            let x = index % 40;
            let y = index / 40;

            if x == 0 || x == 39 || y == 0 || y == 29 {
                initials[index][0] = false;
                initials[index][1] = false;
                initials[index][2] = false;
                initials[index][3] = true;
            }
        }

        initials[1 + 1 * 40][0] = true;
        initials[1 + 1 * 40][1] = false;
        initials[1 + 1 * 40][2] = false;
        initials[1 + 1 * 40][3] = false;

        initials[38 + 28 * 40][0] = true;
        initials[38 + 28 * 40][1] = false;
        initials[38 + 28 * 40][2] = false;
        initials[38 + 28 * 40][3] = false;

        World {
            map: Map::from_data(
                40,
                30,
                gen.gen(40, 30, &constraints, Some(initials)).unwrap(),
            ),
            map_renderer,
        }
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.map_renderer.render(canvas, &self.map);
    }
}
