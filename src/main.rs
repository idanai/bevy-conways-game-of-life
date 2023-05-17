use bevy::prelude::*;
use rand::random;

mod gol;

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

fn parse_program_argumnets() -> gol::SimulationParameters {
	let mut args = std::env::args();

	args.next(); // ignore first parameter- the program's name

	let width = args.next().expect("First parameter is required: width")
		.parse::<u16>().expect("Width must be a natural number")
		as isize;

	let height = args.next().expect("Second parameter is required: height")
		.parse::<u16>().expect("Height must be a natural number")
		as isize;

	gol::SimulationParameters { width, height}
}


fn state_to_color(state: bool) -> Color {
	if state { Color::RED } else { Color::BLACK }
}



fn setup(
	mut commands: Commands,
	window: Query<&Window>,
	simulation_parameters: Res<gol::SimulationParameters>,
) {
	commands.spawn(Camera2dBundle::default());
	let window = window.get_single().unwrap();

	let mut sandbox = gol::Sandbox::new(simulation_parameters.width, simulation_parameters.height)
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

			sandbox.write_cell(gol::Point{x,y}, state);

			commands.spawn((
				gol::Point{x,y},
				gol::NextCellState(state),
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
	mut query: Query<(&gol::NextCellState, &mut Sprite)>,
) {
	for (value, mut sprite) in query.iter_mut() {
		sprite.color = state_to_color(value.0);
	}
}

fn calculate_next_generation(
	mut query: Query<(&gol::Point, &mut gol::NextCellState)>,
	mut sandbox: ResMut<gol::Sandbox>,
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