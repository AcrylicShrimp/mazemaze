extern crate rand;

use super::map::Map;
use crate::render::map_generator::rand::RngCore;

pub fn generate_map(width: u32, height: u32) -> Map {
	let mut maze = vec![1; (width * height) as usize];

	let mut rng = rand::prelude::thread_rng();
	let mut stack: Vec<(i32, i32)> = Vec::new();
	let mut cell_vec: Vec<u8> = Vec::with_capacity(4);

	stack.push((1, 1));

	let is_way = |maze: &Vec<u8>, x: i32, y: i32| -> bool {
		if x < 1 || x as u32 + 1 >= width {
			return false;
		}
		if y < 1 || y as u32 + 1 >= height {
			return false;
		}

		maze[(x as u32 + y as u32 * width) as usize] == 0
	};
	let is_wall = |maze: &Vec<u8>, x: i32, y: i32| -> bool {
		if x < 1 || x as u32 + 1 >= width {
			return false;
		}
		if y < 1 || y as u32 + 1 >= height {
			return false;
		}

		maze[(x as u32 + y as u32 * width) as usize] == 1
	};
	let is_not_juction_horizontal = |maze: &Vec<u8>, x: i32, y: i32| -> bool {
		!is_way(maze, x - 1, y) && !is_way(maze, x + 1, y)
	};
	let is_not_juction_vertical = |maze: &Vec<u8>, x: i32, y: i32| -> bool {
		!is_way(maze, x, y - 1) && !is_way(maze, x, y + 1)
	};

	while !stack.is_empty() {
		let cell = stack.pop().unwrap();

		let u = is_wall(&maze, cell.0, cell.1 - 1)
			&& is_not_juction_horizontal(&maze, cell.0, cell.1 - 1);
		let d = is_wall(&maze, cell.0, cell.1 + 1)
			&& is_not_juction_horizontal(&maze, cell.0, cell.1 + 1);
		let l = is_wall(&maze, cell.0 - 1, cell.1)
			&& is_not_juction_vertical(&maze, cell.0 - 1, cell.1);
		let r = is_wall(&maze, cell.0 + 1, cell.1)
			&& is_not_juction_vertical(&maze, cell.0 + 1, cell.1);

		if !u && !d && !l && !r {
			continue;
		}

		stack.push(cell);

		if u {
			cell_vec.push(0);
		}
		if d {
			cell_vec.push(1);
		}
		if l {
			cell_vec.push(2);
		}
		if r {
			cell_vec.push(3);
		}

		let next_cell = match cell_vec[rng.next_u32() as usize % cell_vec.len()] {
			0 => (cell.0, cell.1 - 1),
			1 => (cell.0, cell.1 + 1),
			2 => (cell.0 - 1, cell.1),
			3 => (cell.0 + 1, cell.1),
			_ => unreachable!(),
		};

		cell_vec.clear();
		stack.push(next_cell);

		maze[(next_cell.0 as u32 + next_cell.1 as u32 * width) as usize] = 0
	}

	Map::from_data(width, height, maze)
}
