use bevy::prelude::*;

#[derive(Component)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point {
	pub x: isize,
	pub y: isize,
}

impl Point {
	pub fn new(x: isize, y: isize) -> Self {
		Self { x, y, }
	}

	pub fn to_index(&self, width: isize) -> isize {
		width * self.y + self.x
	}

	pub fn is_in_bounds(&self, width: isize, height: isize) -> bool {
		(0..width).contains(&self.x) && (0..height).contains(&self.y)
	}
}