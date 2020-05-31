extern crate sdl2;

use super::font_renderer;
use sdl2::pixels;
use sdl2::rect;
use sdl2::render;
use sdl2::video;

pub struct MapRenderer {
	blocks: std::collections::HashMap<u8, (sdl2::render::Texture, u32, u32)>,
	line_height: i32,
}

impl MapRenderer {
	pub fn new(renderer: &font_renderer::FontRenderer) -> Result<MapRenderer, String> {
		let mut blocks = std::collections::HashMap::new();

		blocks.insert(0, renderer.generate_texture(' ')?);
		blocks.insert(
			1,
			renderer.generate_texture_with_color('W', pixels::Color::from((96, 72, 96)))?,
		);
		blocks.insert(
			2,
			renderer.generate_texture_with_color('W', pixels::Color::from((32, 32, 64)))?,
		);
		blocks.insert(
			3,
			renderer.generate_texture_with_color('W', pixels::Color::from((128, 128, 96)))?,
		);

		Ok(MapRenderer {
			blocks,
			line_height: renderer.line_height(),
		})
	}

	pub fn render(
		&self,
		width: u32,
		height: u32,
		blocks: &Vec<u8>,
		canvas: &mut render::Canvas<video::Window>,
	) -> Result<(), String> {
		let mut pos_x = 0i32;
		let mut pos_y = 0i32;

		for y in 0..height {
			for x in 0..width {
				match self.blocks.get(&blocks[(x + y * width) as usize]) {
					Some(block) => {
						canvas.copy(
							&block.0,
							None,
							rect::Rect::new(pos_x, pos_y, block.1, block.2),
						)?;

						pos_x += block.1 as i32 + 4;
					}
					None => {}
				}
			}

			pos_x = 0;
			pos_y += self.line_height;
		}

		Ok(())
	}
}
