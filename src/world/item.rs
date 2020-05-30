#[derive(Clone, Copy)]
pub enum ItemType {
    Equipment,
    Consumable,
    Potion,
    Etc,
}

pub struct Item {
    item_type: ItemType,
    name: String,
    desc: String,
}

impl Item {
    pub fn new(item_type: ItemType, name: String, desc: String) -> Item {
        Item {
            item_type,
            name,
            desc,
        }
    }

    pub fn item_type(&self) -> ItemType {
        self.item_type
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }
}
