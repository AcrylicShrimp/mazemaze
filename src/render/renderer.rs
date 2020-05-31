extern crate sdl2;

use super::font_renderer;
use sdl2::render;
use sdl2::ttf;
use sdl2::video;

pub struct Renderer {
	ttf_context: ttf::Sdl2TtfContext,
	texture_creator: render::TextureCreator<video::WindowContext>,
}

impl Renderer {
	pub fn from(
		ttf_context: ttf::Sdl2TtfContext,
		texture_creator: render::TextureCreator<video::WindowContext>,
	) -> Renderer {
		Renderer {
			ttf_context,
			texture_creator,
		}
	}

	pub fn create_font_renderer<'renderer>(
		&'renderer self,
		font_path: &str,
		font_size: u16,
	) -> Result<font_renderer::FontRenderer<'renderer>, String> {
		let mut font = self.ttf_context.load_font(font_path, font_size)?;

		font.set_hinting(sdl2::ttf::Hinting::Light);

		Ok(font_renderer::FontRenderer::from(
			font,
			&self.texture_creator,
		))
	}
}
