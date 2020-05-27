extern crate sdl2;

use super::super::controller::controller::Controller;
use super::super::controller::player_controller::PlayerController;
use super::super::input::input::Input;
use super::super::network::socket::Socket;
use super::super::object::object::Object;
use super::super::object::player::Player;
use super::map::Map;
use super::map_renderer::MapRenderer;

pub struct World<'render, 'ttf_module, 'rwops> {
    font: &'render sdl2::ttf::Font<'ttf_module, 'rwops>,
    texture_creator: &'render sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    map: Option<Map>,
    map_renderer: MapRenderer,
    players: Vec<Player>,
    player_controller: PlayerController,
}

impl<'render, 'ttf_module, 'rwops> World<'render, 'ttf_module, 'rwops> {
    pub fn new(
        font: &'render sdl2::ttf::Font<'ttf_module, 'rwops>,
        texture_creator: &'render sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> World<'render, 'ttf_module, 'rwops> {
        let mut map_renderer = MapRenderer::new(&font);

        map_renderer.register_block(0, ' ', sdl2::pixels::Color::WHITE, font, texture_creator);
        map_renderer.register_block(
            1,
            'W',
            sdl2::pixels::Color::from((96, 72, 96)),
            font,
            texture_creator,
        );
        map_renderer.register_block(
            2,
            'W',
            sdl2::pixels::Color::from((32, 32, 64)),
            font,
            texture_creator,
        );
        map_renderer.register_block(
            3,
            'W',
            sdl2::pixels::Color::from((128, 128, 96)),
            font,
            texture_creator,
        );

        World {
            font,
            texture_creator,
            map: None,
            map_renderer,
            players: Vec::new(),
            player_controller: PlayerController::new(0.25f32),
        }
    }

    pub fn map(&self) -> &Option<Map> {
        &self.map
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }

    pub fn player_controller(&self) -> &PlayerController {
        &self.player_controller
    }

    pub fn player_controller_mut(&mut self) -> &mut PlayerController {
        &mut self.player_controller
    }

    pub fn update(&mut self, now: std::time::Instant, input: &Input, socket: &mut Socket) {
        match &self.map {
            Some(map) => {
                self.player_controller.update(
                    now,
                    input,
                    map,
                    self.players.first_mut().unwrap().object_mut(),
                    socket,
                );
            }
            None => {}
        }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        match &self.map {
            Some(map) => {
                self.map_renderer.render(canvas, &map);

                for player in self.players.iter() {
                    player.object().render(canvas);
                }
            }
            None => {}
        }
    }

    pub fn add_player(&mut self, id: u64, glyph: char, color: (u8, u8, u8), x: i32, y: i32) {
        self.players.push(Player::new(
            id,
            Object::new(
                x,
                y,
                glyph,
                sdl2::pixels::Color::from(color),
                self.font,
                self.texture_creator,
            ),
        ));
    }

    pub fn remove_player(&mut self, id: u64) {
        self.players.remove(
            self.players
                .iter()
                .position(|player| player.id() == id)
                .unwrap(),
        );
    }

    pub fn init_map(&mut self, map: Map) {
        self.map = Some(map);
    }
}
