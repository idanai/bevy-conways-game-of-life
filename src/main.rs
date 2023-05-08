use bevy::prelude::*;
use rand::random;

const FIRST_SPAWN_CHANCE: u8 = 15;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(parse_program_argumnets())
		.add_startup_system(setup)
		.add_systems((
			calculate_next_generation,
			render_sandbox,
		))
		.run();
}

fn parse_program_argumnets() -> SimulationParameters {
	let mut args = std::env::args();

	args.next(); // ignore first parameter- the program's name

	let width = args.next().expect("First parameter is required: width")
		.parse::<u16>().expect("Width must be a natural number")
		as isize;

	let height = args.next().expect("Second parameter is required: height")
		.parse::<u16>().expect("Height must be a natural number")
		as isize;

	SimulationParameters { width, height}
}

#[derive(Resource)]
struct SimulationParameters {
	pub width: isize,
	pub height: isize,
}

fn state_to_color(state: bool) -> Color {
	if state { Color::RED } else { Color::BLACK }
}

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

#[derive(Component)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NextState(pub bool);

#[derive(Resource)]
pub struct Sandbox {
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

	pub fn get_cell(&self, point: Point) -> &bool {
		&self.cells[point.to_index(self.width) as usize]
	}

	pub fn get_cell_mut(&mut self, point: Point) -> &mut bool {
		&mut self.cells[point.to_index(self.width) as usize]
	}


	pub fn read_cell(&self, point: Point) -> bool {
		*self.get_cell(point)
	}

	pub fn write_cell(&mut self, point: Point, state: bool) {
		*self.get_cell_mut(point) = state;
	}

	pub fn read_moore_neighbourhood(&self, point: Point) -> [bool; 8] {
		if !point.is_in_bounds(self.width, self.height) {
			panic!("Point is outsize of bounds! point = {:?}, bounds = ({}, {})", point, self.width, self.height);
		}

		let mut arr = [false; 8];

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
				arr[neighbor_count] = self.cells[(center_index + offset) as usize];
				neighbor_count += 1;
			}
			neighbor_count += 2 - (x_end - x_start) as usize;
		}
		arr
	}
}

fn setup(
	mut commands: Commands,
	window: Query<&Window>,
	simulation_parameters: Res<SimulationParameters>,
) {
	commands.spawn(Camera2dBundle::default());
	let window = window.get_single().unwrap();

	let mut sandbox = Sandbox::new(simulation_parameters.width, simulation_parameters.height)
		.expect("Area of the world can't be zero or negative");

	let sprite_size = {
		let mut temp = Vec2::new(window.width() / sandbox.width() as f32, window.height() / sandbox.height() as f32);
		if temp.x > temp.y {
			temp.x = temp.y;
		} else {
			temp.y = temp.x;
		}
		temp
	};

	for y in 0..sandbox.height() {
		for x in 0..sandbox.width() {
			let state = random::<u8>() < FIRST_SPAWN_CHANCE;

			sandbox.write_cell(Point{x,y}, state);

			commands.spawn((
				Point{x,y},
				NextState(state),
				SpriteBundle {
					transform: Transform::from_xyz(
						sprite_size.x * (x as f32 - sandbox.width() as f32 * 0.5) + sprite_size.x * 0.5,
						sprite_size.y * (y as f32 - sandbox.height() as f32 * 0.5) + sprite_size.y * 0.5,
						0.),
					sprite: Sprite {
						color: state_to_color(state),
						custom_size: Some(sprite_size),
						..default()
					},
					..default()
				},
			));
		}
	}

	commands.insert_resource(sandbox);
}

fn render_sandbox(
	mut query: Query<(&NextState, &mut Sprite)>,
) {
	for (value, mut sprite) in query.iter_mut() {
		sprite.color = state_to_color(value.0);
	}
}

fn calculate_next_generation(
	mut query: Query<(&Point, &mut NextState)>,
	mut sandbox: ResMut<Sandbox>,
) {
	for (point, mut next_state) in query.iter_mut() {
		let is_alive = next_state.0;
		let moore_neighbourhood = sandbox.read_moore_neighbourhood(*point);
		let neighbors_count = moore_neighbourhood.iter().map(|v| *v as u8).sum::<u8>();
		if is_alive && !(2..=3).contains(&neighbors_count) {
			sandbox.write_cell(*point, false);
			next_state.0 = false;
		} else if neighbors_count == 3 {
			sandbox.write_cell(*point, true);
			next_state.0 = true;
		}
	}
}