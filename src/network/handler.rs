extern crate byteorder;

use byteorder::ReadBytesExt;

pub enum Context {
	WorldReceive {
		width: u32,
		height: u32,
		data: Option<Vec<u8>>,
		player: Option<u32>,
	},
	PlayerReceive,
	PlayerIdReceive,
	MoveReceive,
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
								socket.receive(player_count as usize * 24);
							} else {
								let mut players = vec![];

								for index in 0..player.unwrap() as usize {
									let offset = index * 24;

									let id = std::io::Cursor::new(&received[offset..offset + 8])
										.read_u64::<byteorder::LittleEndian>()
										.unwrap();
									let glyph = std::str::from_utf8(
										&received[offset + 8
											..offset + 8 + received[offset + 12] as usize],
									)
									.unwrap()
									.chars()
									.next()
									.unwrap();
									let color = (
										received[offset + 13],
										received[offset + 14],
										received[offset + 15],
									);
									let x =
										std::io::Cursor::new(&received[offset + 16..offset + 20])
											.read_i32::<byteorder::LittleEndian>()
											.unwrap();
									let y =
										std::io::Cursor::new(&received[offset + 20..offset + 24])
											.read_i32::<byteorder::LittleEndian>()
											.unwrap();

									world.add_player(id, glyph, color, x, y);

									players.push((id, glyph, color, x, y));
								}

								world
									.player_controller_mut()
									.set_player_id(players.first().unwrap().0);

								println!("players: {:?}", players);

								world.init_map(super::super::world::map::Map::from_data(
									*width,
									*height,
									data.take().unwrap(),
								));

								socket.receive(2);
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
			2 => match self.context.as_mut() {
				Some(context) => match context {
					Context::PlayerReceive => match socket.retrieve() {
						Some(received) => {
							let id = std::io::Cursor::new(&received[0..8])
								.read_u64::<byteorder::LittleEndian>()
								.unwrap();
							let glyph =
								std::str::from_utf8(&received[8..8 + received[12] as usize])
									.unwrap()
									.chars()
									.next()
									.unwrap();
							let color = (received[13], received[14], received[15]);
							let x = std::io::Cursor::new(&received[16..20])
								.read_i32::<byteorder::LittleEndian>()
								.unwrap();
							let y = std::io::Cursor::new(&received[20..24])
								.read_i32::<byteorder::LittleEndian>()
								.unwrap();

							world.add_player(id, glyph, color, x, y);

							println!("new player income: {:?}", (id, glyph, color, x, y));

							socket.receive(2);
							self.status = None;
							self.context = None;
						}
						None => {}
					},
					_ => unreachable!(),
				},
				None => {
					socket.receive(24);
					self.context = Some(Context::PlayerReceive);
				}
			},
			3 => match self.context.as_mut() {
				Some(context) => match context {
					Context::PlayerIdReceive => match socket.retrieve() {
						Some(received) => {
							let id = std::io::Cursor::new(&received)
								.read_u64::<byteorder::LittleEndian>()
								.unwrap();

							world.remove_player(id);

							println!("player exit: {:?}", id);

							socket.receive(2);
							self.status = None;
							self.context = None;
						}
						None => {}
					},
					_ => unreachable!(),
				},
				None => {
					socket.receive(8);
					self.context = Some(Context::PlayerIdReceive);
				}
			},
			4 => match self.context.as_mut() {
				Some(context) => match context {
					Context::MoveReceive => match socket.retrieve() {
						Some(received) => {
							let id = std::io::Cursor::new(&received)
								.read_u64::<byteorder::LittleEndian>()
								.unwrap();

							for player in world.players_mut().iter_mut() {
								if player.id() != id {
									continue;
								}

								match received[8] {
									0 => {
										player.object_mut().y -= 1;
									}
									1 => {
										player.object_mut().y += 1;
									}
									2 => {
										player.object_mut().x -= 1;
									}
									3 => {
										player.object_mut().x += 1;
									}
									_ => {}
								}
							}

							println!("player move: {:?} - {}", id, received[8]);

							socket.receive(2);
							self.status = None;
							self.context = None;
						}
						None => {}
					},
					_ => unreachable!(),
				},
				None => {
					socket.receive(9);
					self.context = Some(Context::MoveReceive);
				}
			},
			_ => {
				socket.receive(2);
				self.status = None;
				self.context = None;
			}
		}
	}
}
