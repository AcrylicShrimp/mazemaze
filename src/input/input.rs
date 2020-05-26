extern crate sdl2;

pub struct Input {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn up(&self) -> bool {
        self.up
    }

    pub fn down(&self) -> bool {
        self.down
    }

    pub fn left(&self) -> bool {
        self.left
    }

    pub fn right(&self) -> bool {
        self.right
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event) {
        match event {
            sdl2::event::Event::KeyDown { keycode, .. } => match keycode {
                Some(keycode) => match keycode {
                    sdl2::keyboard::Keycode::Up => {
                        self.up = true;
                    }
                    sdl2::keyboard::Keycode::Down => {
                        self.down = true;
                    }
                    sdl2::keyboard::Keycode::Left => {
                        self.left = true;
                    }
                    sdl2::keyboard::Keycode::Right => {
                        self.right = true;
                    }
                    _ => {}
                },
                _ => {}
            },
            sdl2::event::Event::KeyUp { keycode, .. } => match keycode {
                Some(keycode) => match keycode {
                    sdl2::keyboard::Keycode::Up => {
                        self.up = false;
                    }
                    sdl2::keyboard::Keycode::Down => {
                        self.down = false;
                    }
                    sdl2::keyboard::Keycode::Left => {
                        self.left = false;
                    }
                    sdl2::keyboard::Keycode::Right => {
                        self.right = false;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}
