use super::object::Object;

pub struct Player {
    id: u64,
    object: Object,
}

impl Player {
    pub fn new(id: u64, object: Object) -> Player {
        Player { id, object }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn object(&self) -> &Object {
        &self.object
    }

    pub fn object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
}
