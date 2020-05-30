extern crate sdl2;

use super::super::world::dropped_item;
use super::super::world::item;
use super::renderer;
use sdl2::rect;
use sdl2::render;
use sdl2::video;

pub struct ItemRenderer {
    equipment_texture: (sdl2::render::Texture, u32, u32),
    consumable_texture: (sdl2::render::Texture, u32, u32),
    potion_texture: (sdl2::render::Texture, u32, u32),
    etc_texture: (sdl2::render::Texture, u32, u32),
}

impl ItemRenderer {
    pub fn new(renderer: &renderer::Renderer) -> Result<ItemRenderer, String> {
        Ok(ItemRenderer {
            equipment_texture: renderer.generate_texture('e')?,
            consumable_texture: renderer.generate_texture('c')?,
            potion_texture: renderer.generate_texture('p')?,
            etc_texture: renderer.generate_texture('x')?,
        })
    }

    pub fn render(
        &self,
        items: &Vec<dropped_item::DroppedItem>,
        canvas: &mut render::Canvas<video::Window>,
    ) -> Result<(), String> {
        for item in items.iter() {
            let texture = match item.item().item_type() {
                item::ItemType::Equipment => &self.equipment_texture,
                item::ItemType::Consumable => &self.consumable_texture,
                item::ItemType::Potion => &self.potion_texture,
                item::ItemType::Etc => &self.etc_texture,
            };

            canvas.copy(
                &texture.0,
                None,
                rect::Rect::new(
                    item.x * (texture.1 as i32 + 4),
                    item.y * texture.2 as i32,
                    texture.1,
                    texture.2,
                ),
            )?;
        }

        Ok(())
    }
}
