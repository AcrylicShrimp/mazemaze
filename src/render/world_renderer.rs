extern crate sdl2;

use super::super::world::world;
use super::font_renderer;
use super::item_renderer;
use super::map_renderer;
use super::player_renderer;
use sdl2::render;
use sdl2::video;

pub struct WorldRenderer {
	map_renderer: map_renderer::MapRenderer,
	player_renderer: player_renderer::PlayerRenderer,
	item_renderer: item_renderer::ItemRenderer,
}

impl WorldRenderer {
	pub fn new(renderer: &font_renderer::FontRenderer) -> Result<WorldRenderer, String> {
		Ok(WorldRenderer {
			map_renderer: map_renderer::MapRenderer::new(renderer)?,
			player_renderer: player_renderer::PlayerRenderer::new(renderer)?,
			item_renderer: item_renderer::ItemRenderer::new(renderer)?,
		})
	}

	pub fn render(
		&mut self,
		world: &world::World,
		canvas: &mut render::Canvas<video::Window>,
	) -> Result<(), String> {
		match world.map() {
			Some(map) => {
				self.map_renderer
					.render(map.width(), map.height(), map.data(), canvas)?;
				self.player_renderer.render(world.players(), canvas)?;
				self.item_renderer.render(world.items(), canvas)?;
			}
			None => {}
		}

		Ok(())
	}
}
