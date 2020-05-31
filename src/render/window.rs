extern crate sdl2;

use super::renderer;
use sdl2::render;
use sdl2::ttf;
use sdl2::video;

pub struct Window {
	canvas: render::Canvas<video::Window>,
}

impl Window {
	pub fn from(window: video::Window) -> Result<Window, String> {
		Ok(Window {
			canvas: window
				.into_canvas()
				.present_vsync()
				.index(find_sdl_gl_driver().unwrap())
				.build()
				.map_err(|err| match err {
					sdl2::IntegerOrSdlError::IntegerOverflows(..) => "integer overflows".to_owned(),
					sdl2::IntegerOrSdlError::SdlError(err) => err,
				})?,
		})
	}

	pub fn create_renderer(&self) -> Result<renderer::Renderer, String> {
		Ok(renderer::Renderer::from(
			sdl2::ttf::init().map_err(|err| match err {
				ttf::InitError::AlreadyInitializedError => "".to_owned(),
				ttf::InitError::InitializationError(err) => format!("{}", err),
			})?,
			self.canvas.texture_creator(),
		))
	}

	pub fn canvas(&self) -> &render::Canvas<video::Window> {
		&self.canvas
	}

	pub fn canvas_mut(&mut self) -> &mut render::Canvas<video::Window> {
		&mut self.canvas
	}
}

fn find_sdl_gl_driver() -> Option<u32> {
	for (index, item) in sdl2::render::drivers().enumerate() {
		if item.name == "opengl" {
			return Some(index as u32);
		}
	}

	None
}
