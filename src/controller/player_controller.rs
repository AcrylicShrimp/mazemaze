extern crate sdl2;

use super::super::input::input::Input;
use super::super::network::socket::Socket;
use super::super::world::map::Map;
use super::super::world::player::Player;

use byteorder::WriteBytesExt;

pub struct PlayerController {
    delay: f32,
    player_id: Option<u64>,
    last_move: Option<std::time::Instant>,
}

impl PlayerController {
    pub fn new(delay: f32) -> PlayerController {
        PlayerController {
            delay,
            player_id: None,
            last_move: None,
        }
    }

    pub fn set_player_id(&mut self, player_id: u64) {
        self.player_id = Some(player_id);
    }
}

impl PlayerController {
    pub fn update(
        &mut self,
        now: std::time::Instant,
        input: &Input,
        map: &Map,
        player: &mut Player,
        socket: &mut Socket,
    ) {
        match self.player_id {
            Some(..) => {
                if self.last_move.is_none() {
                    self.last_move = Some(now);
                    return;
                }

                if (now - self.last_move.unwrap()).as_secs_f32() < self.delay {
                    return;
                }

                let mut packet = vec![];

                packet.write_u16::<byteorder::LittleEndian>(2).unwrap();

                if input.up() != input.down() {
                    if input.up() {
                        if map.get_block(player.x as u32, player.y as u32 - 1) == 0 {
                            packet.push(0);
                            socket.send(packet);
                            self.last_move = Some(now);
                        }
                    } else {
                        if map.get_block(player.x as u32, player.y as u32 + 1) == 0 {
                            packet.push(1);
                            socket.send(packet);
                            self.last_move = Some(now);
                        }
                    }
                }

                packet = vec![];

                packet.write_u16::<byteorder::LittleEndian>(2).unwrap();

                if input.left() != input.right() {
                    if input.left() {
                        if map.get_block(player.x as u32 - 1, player.y as u32) == 0 {
                            packet.push(2);
                            socket.send(packet);
                            self.last_move = Some(now);
                        }
                    } else {
                        if map.get_block(player.x as u32 + 1, player.y as u32) == 0 {
                            packet.push(3);
                            socket.send(packet);
                            self.last_move = Some(now);
                        }
                    }
                }
            }
            None => {}
        }
    }
}
