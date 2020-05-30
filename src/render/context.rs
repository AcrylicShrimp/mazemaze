extern crate sdl2;

use super::window;
use sdl2::video;

pub struct Context {
	sdl_context: sdl2::Sdl,
}

impl Context {
	pub fn init() -> Result<Context, String> {
		Ok(Context {
			sdl_context: sdl2::init()?,
		})
	}

	pub fn create_window(
		&self,
		title: &str,
		width: u32,
		height: u32,
	) -> Result<window::Window, String> {
		window::Window::from(
			self.sdl_context
				.video()?
				.window(title, width, height)
				.position_centered()
				.opengl()
				.build()
				.map_err(|err| match err {
					video::WindowBuildError::WidthOverflows(..) => "width overflows".to_owned(),
					video::WindowBuildError::HeightOverflows(..) => "height overflows".to_owned(),
					video::WindowBuildError::InvalidTitle(..) => "invalid title".to_owned(),
					video::WindowBuildError::SdlError(err) => err,
				})?,
		)
	}

	pub fn create_event_pump(&self) -> Result<sdl2::EventPump, String> {
		self.sdl_context.event_pump()
	}
}
