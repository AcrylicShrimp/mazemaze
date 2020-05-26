use super::super::input::input::Input;
use super::super::object::object::Object;
use super::super::world::world::World;

pub trait Controller {
    fn update(
        &mut self,
        now: std::time::Instant,
        input: &Input,
        world: &World,
        object: &mut Object,
    );
}
