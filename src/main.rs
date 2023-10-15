#[cfg(test)]
mod tests;
mod setup;
mod mouse_in;
mod lib;
mod owners;
mod inputs;

use bevy::prelude::*;
use bevy::window::{close_on_esc};
use rand::Rng;
use lib::hex_grid::HexGrid;

#[derive(Resource)]
pub struct Grid(HexGrid<Option<Tile>>);

pub struct Tile {
    pub entity: Entity,
    pub inner_entity: Entity,
    pub owner: Option<Entity>,
}

#[derive(Component)]
struct MainCamera;

fn main() {
    let hg = HexGrid::new_empty(
        30, 10,
        Rect::new(-550.0, -250.0, 550.0, 250.0),
        0.28571428571428573
    );
    
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.07, 0.02, 0.1)))
        .insert_resource(Grid(hg))
        .add_systems(Startup, (
            setup::setup, 
            setup::populate_grid
        ))
        .add_systems(Update, (
            mouse_in::button_inputs, 
            close_on_esc
        ))
        .run();
}
