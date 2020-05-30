use super::item;

pub struct DroppedItem {
    pub x: i32,
    pub y: i32,
    item: item::Item,
}

impl DroppedItem {
    pub fn new(x: i32, y: i32, item: item::Item) -> DroppedItem {
        DroppedItem { x, y, item }
    }

    pub fn item(&self) -> &item::Item {
        &self.item
    }

    pub fn into_item(self) -> item::Item {
        self.item
    }
}
