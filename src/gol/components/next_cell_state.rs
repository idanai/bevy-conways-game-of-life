use bevy::prelude::*;

#[derive(Component)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NextCellState(pub bool);