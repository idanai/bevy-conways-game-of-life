#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Cell {
	#[default]
	Dead,
	Alive(i8),
}
use Cell::*;

impl Cell {
	pub fn is_dead(&self) -> bool {
		match self {
			Dead => true,
			_ => false,
		}
	}

	pub fn is_alive(&self) -> bool {
		!self.is_dead()
	}

	pub fn as_option(&self) -> Option<&i8> {
		match self {
			Alive(value) => Some(value),
			_ => None,
		}
	}
}