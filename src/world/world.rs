extern crate sdl2;

use super::super::controller::controller::Controller;
use super::super::controller::player_controller::PlayerController;
use super::super::input::input::Input;
use super::super::network::socket::Socket;
use super::super::object::player::Player;
use super::map::Map;

pub struct World {
    map: Option<Map>,
    players: Vec<Player>,
    player_controller: PlayerController,
}

impl World {
    pub fn new() -> World {
        World {
            map: None,
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
        // match &self.map {
        //     Some(map) => {
        //         self.player_controller.update(
        //             now,
        //             input,
        //             map,
        //             self.players.first_mut().unwrap().object_mut(),
        //             socket,
        //         );
        //     }
        //     None => {}
        // }
    }

    pub fn add_player(&mut self, id: u64, glyph: char, color: (u8, u8, u8), x: i32, y: i32) {
        // self.players.push(Player::new(
        //     id,
        //     Object::new(
        //         x,
        //         y,
        //         glyph,
        //         sdl2::pixels::Color::from(color),
        //         self.font,
        //         self.texture_creator,
        //     ),
        // ));
    }

    pub fn remove_player(&mut self, id: u64) {
        // self.players.remove(
        //     self.players
        //         .iter()
        //         .position(|player| player.id() == id)
        //         .unwrap(),
        // );
    }

    pub fn init_map(&mut self, map: Map) {
        self.map = Some(map);
    }
}
