extern crate sdl2;

use super::renderer;
use sdl2::render;
use sdl2::ttf;
use sdl2::video;

pub struct Window {
	canvas: render::Canvas<video::Window>,
	ttf_context: ttf::Sdl2TtfContext,
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
			ttf_context: sdl2::ttf::init().map_err(|err| match err {
				ttf::InitError::AlreadyInitializedError => "".to_owned(),
				ttf::InitError::InitializationError(err) => format!("{}", err),
			})?,
		})
	}

	pub fn create_renderer<'ttf>(
		&'ttf self,
		font_path: &str,
		font_size: u16,
	) -> Result<renderer::Renderer<'ttf>, String> {
		renderer::Renderer::from(
			self.canvas.texture_creator(),
			&self.ttf_context,
			font_path,
			font_size,
		)
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
