extern crate sdl2;

use super::super::world::world;
use super::map_renderer;
use super::renderer;
use sdl2::render;
use sdl2::video;

pub struct WorldRenderer {
	map_renderer: map_renderer::MapRenderer,
}

impl WorldRenderer {
	pub fn new(renderer: &renderer::Renderer) -> Result<WorldRenderer, String> {
		Ok(WorldRenderer {
			map_renderer: map_renderer::MapRenderer::new(renderer)?,
		})
	}

	pub fn render(
		&self,
		world: &world::World,
		canvas: &mut render::Canvas<video::Window>,
	) -> Result<(), String> {
		match world.map() {
			Some(map) => self
				.map_renderer
				.render(map.width(), map.height(), map.data(), canvas)?,
			None => {}
		}

		Ok(())
	}
}
