extern crate sdl2;

use super::super::object::player::Player;
use super::renderer;
use sdl2::rect;
use sdl2::render;
use sdl2::video;

pub struct PlayerRenderer {
    texture: (sdl2::render::Texture, u32, u32),
}

impl PlayerRenderer {
    pub fn new(renderer: &renderer::Renderer) -> Result<PlayerRenderer, String> {
        Ok(PlayerRenderer {
            texture: renderer.generate_texture('@')?,
        })
    }

    pub fn render(
        &self,
        players: &Vec<Player>,
        canvas: &mut render::Canvas<video::Window>,
    ) -> Result<(), String> {
        for player in players.iter() {
            canvas.copy(
                &self.texture.0,
                None,
                rect::Rect::new(
                    player.object().x * (self.texture.1 as i32 + 4),
                    player.object().y * self.texture.2 as i32,
                    self.texture.1,
                    self.texture.2,
                ),
            )?;
        }

        Ok(())
    }
}
