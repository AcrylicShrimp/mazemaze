extern crate sdl2;

mod render;
use render::map_generator::generate_map;
use render::map_renderer::MapRenderer;

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
        .window("Maze Maze", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let mut map = generate_map(30, 16);
    let map_renderer = MapRenderer::new(&canvas);

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main_loop,
                sdl2::event::Event::KeyDown { keycode, .. } => match keycode {
                    Some(keycode) => {
                        if keycode == sdl2::keyboard::Keycode::R {
                            map = generate_map(30, 16);
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
        }

        canvas.clear();

        // TODO: Render something here.
        map_renderer.render(&mut canvas, &map);

        canvas.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
