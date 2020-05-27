use super::super::input::input::Input;
use super::super::network::socket::Socket;
use super::super::object::object::Object;
use super::super::world::map::Map;

pub trait Controller {
    fn update(
        &mut self,
        now: std::time::Instant,
        input: &Input,
        map: &Map,
        object: &mut Object,
        socket: &mut Socket,
    );
}
