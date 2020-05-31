extern crate sdl2;

use sdl2::pixels;
use sdl2::render;
use sdl2::ttf;
use sdl2::video;

pub struct FontRenderer<'renderer> {
    font: ttf::Font<'renderer, 'static>,
    texture_creator: &'renderer render::TextureCreator<video::WindowContext>,
}

impl<'renderer> FontRenderer<'renderer> {
    pub fn from(
        font: ttf::Font<'renderer, 'static>,
        texture_creator: &'renderer render::TextureCreator<video::WindowContext>,
    ) -> FontRenderer<'renderer> {
        FontRenderer {
            font,
            texture_creator,
        }
    }

    pub fn line_height(&self) -> i32 {
        self.font.height()
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

        Ok((texture, surface.width(), surface.height()))
    }

    pub fn generate_text_texture(
        &mut self,
        text: &str,
        max_width: u32,
        style: ttf::FontStyle,
    ) -> Result<(render::Texture, u32, u32), String> {
        self.generate_text_texture_with_color(text, max_width, style, pixels::Color::WHITE)
    }

    pub fn generate_text_texture_with_color(
        &mut self,
        text: &str,
        max_width: u32,
        style: ttf::FontStyle,
        color: pixels::Color,
    ) -> Result<(render::Texture, u32, u32), String> {
        self.font.set_style(style);

        let surface = self
            .font
            .render(text)
            .blended_wrapped(color, max_width)
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

        Ok((texture, surface.width(), surface.height()))
    }
}
