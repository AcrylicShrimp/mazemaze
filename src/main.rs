extern crate sdl2;

mod controller;
mod input;
mod object;
mod world;

use controller::controller::Controller;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }

    None
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Maze Maze", 802, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context
        .load_font("assets/fonts/Inconsolata.ttf", 16)
        .unwrap();

    font.set_hinting(sdl2::ttf::Hinting::Light);

    let world = world::world::World::new(&font, &texture_creator);
    let mut character = object::object::Object::new(
        1,
        1,
        '@',
        sdl2::pixels::Color::CYAN,
        &font,
        &texture_creator,
    );
    let mut player_controller = controller::player_controller::PlayerController::new(0.25f32);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut input = input::input::Input::new();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    'main_loop: loop {
        let now = std::time::Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main_loop,
                _ => {
                    input.handle_event(&event);
                }
            }
        }

        player_controller.update(now, &input, &world, &mut character);

        canvas.clear();

        // TODO: Render something here.
        world.render(&mut canvas);
        character.render(&mut canvas);

        canvas.present();
    }
}
