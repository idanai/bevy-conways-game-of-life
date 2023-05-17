use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationParameters {
	pub width: isize,
	pub height: isize,
}