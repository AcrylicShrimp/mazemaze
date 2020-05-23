extern crate sdl2;

use super::map::Map;

pub struct MapRenderer {
	block_map: std::collections::HashMap<u8, (sdl2::render::Texture, u32, u32)>,
	line_height: u32,
}

impl MapRenderer {
	pub fn new(canvas: &sdl2::render::Canvas<sdl2::video::Window>) -> MapRenderer {
		let texture_creator = canvas.texture_creator();
		let mut block_map: std::collections::HashMap<u8, (sdl2::render::Texture, u32, u32)> =
			std::collections::HashMap::new();

		let ttf_context = sdl2::ttf::init().unwrap();
		let mut font = ttf_context
			.load_font("assets/fonts/Inconsolata.ttf", 1)
			.unwrap();

		font.set_hinting(sdl2::ttf::Hinting::Light);

		let generate_texture = |glyph: char| -> sdl2::render::Texture {
			let texture = texture_creator
				.create_texture_from_surface(
					&font
						.render_char(glyph)
						.blended(sdl2::pixels::Color::WHITE)
						.unwrap(),
				)
				.unwrap();

			texture
		};

		block_map.insert(
			0,
			(
				generate_texture(' '),
				font.size_of_char(' ').unwrap().0,
				font.size_of_char(' ').unwrap().1,
			),
		);
		block_map.insert(
			1,
			(
				generate_texture('W'),
				font.size_of_char('W').unwrap().0,
				font.size_of_char('W').unwrap().1,
			),
		);

		MapRenderer {
			block_map,
			line_height: font.height() as u32,
		}
	}

	pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, map: &Map) {
		let mut pos_x: u32 = 0;
		let mut pos_y: u32 = 0;

		for y in 0..map.get_height() {
			for x in 0..map.get_width() {
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
}
