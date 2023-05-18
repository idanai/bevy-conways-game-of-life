use bevy::prelude::*;
use crate::gol::*;

#[derive(Resource)]
pub struct Sandbox {
	cells: Vec<Cell>,
	width: isize,
	height: isize,
}

impl Sandbox {
	pub fn new(width: isize, height: isize) -> Option<Self> {
		if width <= 0 || height <= 0 {
			return None;
		}
		Some(Self {
			cells: vec![Cell::Dead; (width * height) as usize],
			width,
			height
		})
	}

	pub fn width(&self) -> isize { self.width }

	pub fn height(&self) -> isize { self.height }

	pub fn area(&self) -> isize { self.width * self.height }

	pub fn get_cell(&self, point: Point) -> &Cell {
		&self.cells[point.to_index(self.width) as usize]
	}

	pub fn get_cell_mut(&mut self, point: Point) -> &mut Cell {
		&mut self.cells[point.to_index(self.width) as usize]
	}

	pub fn write_cell(&mut self, point: Point, cell: Cell) {
		*self.get_cell_mut(point) = cell;
	}

	pub fn read_moore_neighbourhood(&self, point: Point) -> [Cell; 8] {
		if !point.is_in_bounds(self.width, self.height) {
			panic!("Point is outsize of bounds! point = {:?}, bounds = ({}, {})", point, self.width, self.height);
		}

		let mut arr = [Cell::Dead; 8];

		let mut neighbor_count = 0;

		let x_start = if point.x == 0 { neighbor_count += 1; 0 } else { -1 };
		let y_start = if point.y == 0 { neighbor_count += 3; 0 } else { -1 };
		let x_end = if point.x == self.width-1 { 0 } else { 1 };
		let y_end = if point.y == self.height-1 { 0 } else { 1 };

		let center_index = point.to_index(self.width);
		for y in y_start..=y_end {
			for x in x_start..=x_end {
				if y == 0 && x == 0 {
					continue;
				}
				let offset = Point{x,y}.to_index(self.width);
				arr[neighbor_count] = self.cells[(center_index + offset) as usize].clone();
				neighbor_count += 1;
			}
			neighbor_count += 2 - (x_end - x_start) as usize;
		}
		arr
	}
}
