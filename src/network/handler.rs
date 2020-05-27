extern crate byteorder;

use byteorder::ReadBytesExt;

pub enum Context {
	WorldReceive {
		width: u32,
		height: u32,
		data: Option<Vec<u8>>,
		player: Option<u32>,
	},
}

pub struct Handler {
	status: Option<u16>,
	context: Option<Context>,
}

impl Handler {
	pub fn new() -> Handler {
		Handler {
			status: None,
			context: None,
		}
	}

	pub fn handle_socket(
		&mut self,
		socket: &mut super::socket::Socket,
		world: &mut super::super::world::world::World,
	) {
		let status_code: u16;

		{
			if self.status.is_none() {
				let received = socket.retrieve();

				if received.is_none() {
					return;
				}

				self.status = Some(
					std::io::Cursor::new(received.unwrap())
						.read_u16::<byteorder::LittleEndian>()
						.unwrap(),
				);
			}

			status_code = self.status.unwrap();
		}

		self.handle_packet(status_code, socket, world);
	}

	fn handle_packet(
		&mut self,
		status: u16,
		socket: &mut super::socket::Socket,
		world: &mut super::super::world::world::World,
	) {
		println!("status={}", status);

		match status {
			1 => match self.context.as_mut() {
				Some(context) => match context {
					Context::WorldReceive {
						width,
						height,
						data,
						player,
					} => match socket.retrieve() {
						Some(received) => {
							if data.is_none() {
								*data = Some(received);
								socket.receive(4);
							} else if player.is_none() {
								let player_count = std::io::Cursor::new(&received)
									.read_u32::<byteorder::LittleEndian>()
									.unwrap();

								*player = Some(player_count);
								socket.receive(player_count as usize * 16);
							} else {
								let mut player_positions = vec![];

								for index in 0..player.unwrap() as usize {
									player_positions.push((
										std::io::Cursor::new(&received[index * 16..index * 16 + 8])
											.read_u64::<byteorder::LittleEndian>()
											.unwrap(),
										(
											std::io::Cursor::new(
												&received[index * 16 + 8..index * 16 + 12],
											)
											.read_u32::<byteorder::LittleEndian>()
											.unwrap(),
											std::io::Cursor::new(
												&received[index * 16 + 12..(index + 1) * 16],
											)
											.read_u32::<byteorder::LittleEndian>()
											.unwrap(),
										),
									));
								}

								world.init_map(super::super::world::map::Map::from_data(
									*width,
									*height,
									data.take().unwrap(),
								));

								self.status = None;
								self.context = None;
							}
						}
						None => {}
					},
					_ => unreachable!(),
				},
				None => match socket.retrieve() {
					Some(received) => {
						let width = std::io::Cursor::new(&received[0..4])
							.read_u32::<byteorder::LittleEndian>()
							.unwrap();
						let height = std::io::Cursor::new(&received[4..8])
							.read_u32::<byteorder::LittleEndian>()
							.unwrap();

						self.context = Some(Context::WorldReceive {
							width,
							height,
							data: None,
							player: None,
						});

						socket.receive((width * height) as usize);
					}
					None => {
						socket.receive(8);
					}
				},
			},
			2 => {}
			3 => {}
			_ => {
				socket.receive(2);
				self.status = None;
				self.context = None;
			}
		}
	}
}
