
const WIDTH: isize = 500;
const HEIGHT: isize = 500;

fn main() {
	// TEST
	let mut sandbox = Sandbox::new(3, 3).expect("Failed to create world");
	sandbox.write_cell(Point::new(0, 0), true);
	sandbox.write_cell(Point::new(1, 0), true);
	sandbox.write_cell(Point::new(2, 0), true);
	sandbox.write_cell(Point::new(0, 1), true);
	sandbox.write_cell(Point::new(1, 1), true);
	sandbox.write_cell(Point::new(2, 1), true);
	sandbox.write_cell(Point::new(0, 2), true);
	sandbox.write_cell(Point::new(1, 2), true);
	sandbox.write_cell(Point::new(2, 2), true);

	println!("Whole Grid:");
	for y in 0..sandbox.height() {
		for x in 0..sandbox.width() {
			print!("{}", if sandbox.read_cell(Point{x,y}) {'O'} else {'-'} );
		}
		println!();
	}

	println!("Frame:");

	sandbox.read_moore_neighbourhood(Point::new(2,0))
	.iter().enumerate().for_each(|(index, value)| {
		if(index == 3 ||index == 5) {
			println!();
		}
		if (index == 4) {
			print!(" ");
		}
		print!("{}", if *value {'X'} else {'-'});
	});
	println!();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
	pub x: isize,
	pub y: isize,
}

impl Point {
	pub fn new(x: isize, y: isize) -> Self {
		Self { x, y, }
	}
}


struct Sandbox {
	cells: Vec<bool>,
	width: isize,
	height: isize,
}

impl Sandbox {
	pub fn new(width: isize, height: isize) -> Option<Self> {
		if width <= 0 || height <= 0 {
			return None;
		}
		Some(Self {
			cells: vec![false; (width * height) as usize],
			width,
			height
		})
	}

	pub fn width(&self) -> isize { self.width }

	pub fn height(&self) -> isize { self.height }

	pub fn area(&self) -> isize { self.width * self.height }

	fn xy_to_index(width: isize, x: isize, y: isize) -> isize {
		x + y * width
	}

	fn point_to_index(width: isize, point: Point) -> isize {
		Self::xy_to_index(width, point.x, point.y)
	}

	pub fn read_cell(&self, point: Point) -> bool {
		self.cells[Self::point_to_index(self.width, point) as usize]
	}

	pub fn write_cell(&mut self, point: Point, state: bool) {
		self.cells[Self::point_to_index(self.width, point) as usize] = state;
	}
	
	fn is_in_bounds(&self, x: isize, y: isize) -> bool {
		y >= 0 && y < self.height && x >= 0 || x < self.width
	}

	pub fn read_moore_neighbourhood(&self, point: Point) -> [bool; 8] {
		if !self.is_in_bounds(point.x, point.y) {
			panic!("Point is outsize of bounds! point = {:?}, bounds = ({}, {})", point, self.width, self.height);
		}

		let mut arr = [false; 8];

		let mut neighbor_count = 0;

		let x_start = if point.x == 0 { neighbor_count += 1; 0 } else { -1 };
		let y_start = if point.y == 0 { neighbor_count += 3; 0 } else { -1 };
		let x_end = if point.x == self.width-1 { 0 } else { 1 };
		let y_end = if point.y == self.height-1 { 0 } else { 1 };

		let center_index = Self::point_to_index(self.width, point);
		for y in y_start..=y_end {
			for x in x_start..=x_end {
				if y == 0 && x == 0 {
					continue;
				}
				let offset = Self::xy_to_index(self.width, x, y);
				arr[neighbor_count] = self.cells[(center_index + offset) as usize];
				neighbor_count += 1;
			}
			neighbor_count += 2 - (x_end - x_start) as usize;
		}
		arr
	}
}