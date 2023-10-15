

mod populate_grid;

pub use self::populate_grid::*;
use bevy::prelude::*;
use crate::{MainCamera};

pub(crate) fn setup(
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
