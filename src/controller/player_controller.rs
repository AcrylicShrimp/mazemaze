extern crate sdl2;

use super::super::input::input::Input;
use super::super::object::object::Object;
use super::super::world::world::World;
use super::controller::Controller;

pub struct PlayerController {
    delay: f32,
    last_move: Option<std::time::Instant>,
}

impl PlayerController {
    pub fn new(delay: f32) -> PlayerController {
        PlayerController {
            delay,
            last_move: None,
        }
    }
}

impl Controller for PlayerController {
    fn update(
        &mut self,
        now: std::time::Instant,
        input: &Input,
        world: &World,
        object: &mut Object,
    ) {
        match world.map() {
            Some(map) => {
                if self.last_move.is_none() {
                    self.last_move = Some(now);
                    return;
                }
                if (now - self.last_move.unwrap()).as_secs_f32() < self.delay {
                    return;
                }
                if input.up() != input.down() {
                    if input.up() {
                        if map.get_block(object.x as u32, object.y as u32 - 1) == 0 {
                            object.y -= 1;
                            self.last_move = Some(now);
                        }
                    } else {
                        if map.get_block(object.x as u32, object.y as u32 + 1) == 0 {
                            object.y += 1;
                            self.last_move = Some(now);
                        }
                    }
                }
                if input.left() != input.right() {
                    if input.left() {
                        if map.get_block(object.x as u32 - 1, object.y as u32) == 0 {
                            object.x -= 1;
                            self.last_move = Some(now);
                        }
                    } else {
                        if map.get_block(object.x as u32 + 1, object.y as u32) == 0 {
                            object.x += 1;
                            self.last_move = Some(now);
                        }
                    }
                }
            }
            None => {}
        }
    }
}
