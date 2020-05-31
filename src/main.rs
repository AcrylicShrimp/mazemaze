extern crate byteorder;
extern crate sdl2;

mod controller;
mod input;
mod network;
mod render;
mod world;

use byteorder::WriteBytesExt;

fn main() -> Result<(), String> {
    let context = render::context::Context::init()?;
    let mut window = context.create_window("Maze Maze", 802, 600)?;
    let renderer = window.create_renderer()?;
    let font_renderer = renderer.create_font_renderer("assets/fonts/Inconsolata.ttf", 18)?;

    let stream = std::net::TcpStream::connect("127.0.0.1:19980").unwrap();
    stream.set_nodelay(true).unwrap();
    stream.set_nonblocking(true).unwrap();

    let mut socket = network::socket::Socket::from(stream);
    let mut handler = network::handler::Handler::new();

    {
        let mut packet = vec![];
        packet.write_u16::<byteorder::LittleEndian>(1).unwrap();
        socket.send(packet);
        socket.receive(2);
    }

    let mut input = input::input::Input::new();
    let mut world = world::world::World::new();
    let mut world_renderer = render::world_renderer::WorldRenderer::new(&font_renderer)?;
    let mut ui_renderer = render::ui_renderer::UIRenderer::new(&font_renderer)?;

    let mut canvas = window.canvas_mut();
    let mut event_pump = context.create_event_pump()?;

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    'main_loop: loop {
        let now = std::time::Instant::now();

        for _ in 0..100 {
            if !socket.update() {
                return Err("unable to update socket".to_owned());
            }

            handler.handle_socket(&mut socket, &mut world);
        }

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main_loop,
                _ => {
                    input.handle_event(&event);
                }
            }
        }

        ui_renderer.mode = if input.inventory() {
            render::ui_renderer::UIRendererMode::Inventory
        } else {
            render::ui_renderer::UIRendererMode::InGame
        };

        ui_renderer.update(&input);
        world.update(now, &input, &mut socket);

        canvas.clear();

        // TODO: Render something here.
        match ui_renderer.mode {
            render::ui_renderer::UIRendererMode::InGame => {
                world_renderer.render(&world, &mut canvas)?;
                ui_renderer.render(&world, &mut canvas, &mut font_renderer)?;
            }
            render::ui_renderer::UIRendererMode::Inventory => {
                ui_renderer.render(&world, &mut canvas, &mut font_renderer)?;
            }
        }

        canvas.present();
    }

    Ok(())
}
