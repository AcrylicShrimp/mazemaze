extern crate sdl2;

use sdl2::pixels;
use sdl2::render;
use sdl2::ttf;
use sdl2::video;

pub struct Renderer<'ttf> {
	multiplier: u16,
	texture_creator: render::TextureCreator<video::WindowContext>,
	font: ttf::Font<'ttf, 'static>,
}

impl<'ttf> Renderer<'ttf> {
	pub fn from(
		multiplier: u16,
		texture_creator: render::TextureCreator<video::WindowContext>,
		ttf_context: &'ttf ttf::Sdl2TtfContext,
		font_path: &str,
		font_size: u16,
	) -> Result<Renderer<'ttf>, String> {
		let mut font = ttf_context.load_font(font_path, font_size * multiplier)?;

		font.set_hinting(sdl2::ttf::Hinting::Light);

		Ok(Renderer {
			multiplier,
			texture_creator,
			font,
		})
	}

	pub fn line_height(&self) -> i32 {
		self.font.height() / self.multiplier as i32
	}

	pub fn generate_texture(&self, glyph: char) -> Result<(render::Texture, u32, u32), String> {
		self.generate_texture_with_color(glyph, pixels::Color::WHITE)
	}

	pub fn generate_texture_with_color(
		&self,
		glyph: char,
		color: pixels::Color,
	) -> Result<(render::Texture, u32, u32), String> {
		let surface = self
			.font
			.render_char(glyph)
			.blended(color)
			.map_err(|err| match err {
				ttf::FontError::InvalidLatin1Text(..) => {
					"this font does not contain that glyph".to_owned()
				}
				ttf::FontError::SdlError(err) => err,
			})?;
		let texture = self
			.texture_creator
			.create_texture_from_surface(&surface)
			.map_err(|err| match err {
				render::TextureValueError::WidthOverflows(..) => "width overflows".to_owned(),
				render::TextureValueError::HeightOverflows(..) => "height overflows".to_owned(),
				render::TextureValueError::WidthMustBeMultipleOfTwoForFormat(..) => {
					"width must be multiple of 2".to_owned()
				}
				render::TextureValueError::SdlError(err) => err,
			})?;

		Ok((
			texture,
			surface.width() / self.multiplier as u32,
			surface.height() / self.multiplier as u32,
		))
	}
}
