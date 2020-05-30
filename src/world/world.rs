extern crate sdl2;

use super::super::controller::player_controller::PlayerController;
use super::super::input::input::Input;
use super::super::network::socket::Socket;
use super::dropped_item;
use super::item;
use super::map::Map;
use super::player;

pub struct World {
    map: Option<Map>,
    items: Vec<dropped_item::DroppedItem>,
    players: Vec<player::Player>,
    player_controller: PlayerController,
}

impl World {
    pub fn new() -> World {
        World {
            map: None,
            items: vec![dropped_item::DroppedItem::new(
                1,
                1,
                item::Item::new(
                    item::ItemType::Equipment,
                    "test!".to_owned(),
                    "This is a test item.".to_owned(),
                ),
            )],
            players: Vec::new(),
            player_controller: PlayerController::new(0.25f32),
        }
    }

    pub fn map(&self) -> &Option<Map> {
        &self.map
    }

    pub fn items(&self) -> &Vec<dropped_item::DroppedItem> {
        &self.items
    }

    pub fn players(&self) -> &Vec<player::Player> {
        &self.players
    }

    pub fn players_mut(&mut self) -> &mut Vec<player::Player> {
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
                    self.players.first_mut().unwrap(),
                    socket,
                );
            }
            None => {}
        }
    }

    pub fn add_player(&mut self, id: u64, color: (u8, u8, u8), x: i32, y: i32) {
        self.players.push(player::Player::new(
            id,
            x,
            y,
            sdl2::pixels::Color::from(color),
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
