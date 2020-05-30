extern crate sdl2;

use super::super::world::world;
use super::map_renderer;
use super::player_renderer;
use super::renderer;
use sdl2::render;
use sdl2::video;

pub struct WorldRenderer {
	map_renderer: map_renderer::MapRenderer,
	player_renderer: player_renderer::PlayerRenderer,
}

impl WorldRenderer {
	pub fn new(renderer: &renderer::Renderer) -> Result<WorldRenderer, String> {
		Ok(WorldRenderer {
			map_renderer: map_renderer::MapRenderer::new(renderer)?,
			player_renderer: player_renderer::PlayerRenderer::new(renderer)?,
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
			}
			None => {}
		}

		Ok(())
	}
}
