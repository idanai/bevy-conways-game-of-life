use bevy::prelude::*;
use crate::gol::Cell;

#[derive(Component)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NextCellState(pub Cell);