extern crate sdl2;

use super::super::input::input;
use super::super::world::world;
use super::font_renderer;
use sdl2::rect;
use sdl2::render;
use sdl2::ttf;
use sdl2::video;

pub enum UIRendererMode {
    InGame,
    Inventory,
}

pub struct UIRenderer {
    pub mode: UIRendererMode,
    cursor_texture: (render::Texture, u32, u32),
    splitter_texture: (render::Texture, u32, u32),
    selected_item_index: usize,
}

impl UIRenderer {
    pub fn new(renderer: &font_renderer::FontRenderer) -> Result<UIRenderer, String> {
        Ok(UIRenderer {
            mode: UIRendererMode::InGame,
            cursor_texture: renderer.generate_texture('>')?,
            splitter_texture: renderer.generate_texture('|')?,
            selected_item_index: 0,
        })
    }

    pub fn update(&mut self, input: &input::Input) {
        if input.up() {
            if self.selected_item_index != 0 {
                self.selected_item_index -= 1;
            }
        }

        if input.down() {
            self.selected_item_index += 1;
        }
    }

    pub fn render(
        &self,
        world: &world::World,
        canvas: &mut render::Canvas<video::Window>,
        renderer: &mut font_renderer::FontRenderer,
    ) -> Result<(), String> {
        match self.mode {
            UIRendererMode::InGame => {}
            UIRendererMode::Inventory => {
                let offset_y = 600 % self.splitter_texture.2 as i32 / 2;
                let extra_offset_y = (600 - offset_y * 2) % self.splitter_texture.2 as i32;

                for y in (offset_y..600 - offset_y - extra_offset_y)
                    .step_by(self.splitter_texture.2 as usize)
                {
                    canvas.copy(
                        &self.splitter_texture.0,
                        None,
                        rect::Rect::new(
                            400 - self.splitter_texture.1 as i32 / 2,
                            y,
                            self.splitter_texture.1,
                            self.splitter_texture.2,
                        ),
                    )?;
                }

                let padding = 14;
                let cursor_padding = 8;
                let max_message_width =
                    (400 - padding * 2 - cursor_padding - self.cursor_texture.1 as i32) as u32;

                let messages = vec![
                    "Wan So Nam's Maggot Jaffaghetti",
                    "Kim Hurse's Cabbage Kimchi Noodles",
                    "Piper's Twisted Nether Waste A Course",
                    "Wan So Nam's Maggot Jaffaghetti",
                    "Kim Hurse's Cabbage Kimchi Noodles",
                    "Piper's Twisted Nether Waste A Course",
                    "Wan So Nam's Maggot Jaffaghetti",
                    "Kim Hurse's Cabbage Kimchi Noodles",
                    "Piper's Twisted Nether Waste A Course",
                    "Wan So Nam's Maggot Jaffaghetti",
                    "Kim Hurse's Cabbage Kimchi Noodles",
                    "Piper's Twisted Nether Waste A Course",
                    "Wan So Nam's Maggot Jaffaghetti",
                    "Kim Hurse's Cabbage Kimchi Noodles",
                    "Piper's Twisted Nether Waste A Course",
                    "Wan So Nam's Maggot Jaffaghetti",
                    "Kim Hurse's Cabbage Kimchi Noodles",
                    "Piper's Twisted Nether Waste A Course",
                    "Wan So Nam's Maggot Jaffaghetti",
                    "Kim Hurse's Cabbage Kimchi Noodles",
                    "Piper's Twisted Nether Waste A Course",
                    "Wan So Nam's Maggot Jaffaghetti",
                    "Kim Hurse's Cabbage Kimchi Noodles",
                    "Piper's Twisted Nether Waste A Course",
                    "Wan So Nam's Maggot Jaffaghetti",
                    "Kim Hurse's Cabbage Kimchi Noodles",
                    "Piper's Twisted Nether Waste A Course",
                    "Wan So Nam's Maggot Jaffaghetti",
                    "Kim Hurse's Cabbage Kimchi Noodles",
                    "Piper's Twisted Nether Waste A Course",
                ];
                // let messages = vec![];

                if messages.is_empty() {
                    let message = renderer.generate_text_texture(
                        "You have no item!",
                        max_message_width + cursor_padding as u32 + self.cursor_texture.1,
                        ttf::FontStyle::ITALIC,
                    )?;

                    canvas.copy(
                        &message.0,
                        None,
                        rect::Rect::new(padding, padding, message.1, message.2),
                    )?;
                } else {
                    let mut y = padding;

                    canvas.copy(
                        &self.cursor_texture.0,
                        None,
                        rect::Rect::new(padding, y, self.cursor_texture.1, self.cursor_texture.2),
                    )?;

                    for index in self.selected_item_index..messages.len() {
                        let message = renderer.generate_text_texture(
                            messages[index],
                            max_message_width,
                            if index == self.selected_item_index {
                                ttf::FontStyle::UNDERLINE
                            } else {
                                ttf::FontStyle::NORMAL
                            },
                        )?;

                        canvas.copy(
                            &message.0,
                            None,
                            rect::Rect::new(
                                padding + cursor_padding + self.cursor_texture.1 as i32,
                                y,
                                message.1,
                                message.2,
                            ),
                        )?;

                        y += message.2 as i32 + padding;
                    }
                }
            }
        }

        Ok(())
    }
}
