pub struct Map {
	data: Vec<u8>,
	width: u32,
	height: u32,
}

impl Map {
	pub fn new(width: u32, height: u32) -> Map {
		Map {
			data: vec![1; (width * height) as usize],
			width,
			height,
		}
	}

	pub fn from_data(width: u32, height: u32, data: Vec<u8>) -> Map {
		Map {
			data,
			width,
			height,
		}
	}

	pub fn get_block(&self, x: u32, y: u32) -> u8 {
		self.data[(x + y * self.width) as usize]
	}

	pub fn get_width(&self) -> u32 {
		self.width
	}

	pub fn get_height(&self) -> u32 {
		self.height
	}
}
