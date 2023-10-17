#[cfg(test)]
mod tests;
mod setup;
mod lib;
mod owners;
mod inputs;

use bevy::prelude::*;
use bevy::window::{close_on_esc};
use lib::hex_grid::HexGrid;
use crate::inputs::turn_press;

#[derive(Resource)]
pub struct Grid(HexGrid<Option<Tile>>);

pub struct Tile {
    pub entity: Entity,
    pub inner_entity: Entity,
    pub owner: Option<Entity>,
}

#[derive(Resource)]
pub struct PlayerOwner<'p>(Option<&'p Entity>);

#[derive(Component)]
pub struct MainCamera;



fn main() {
    let hg = HexGrid::new_empty(
        60, 30,
        Rect::new(-550.0, -250.0, 550.0, 250.0),
        0.28571428571428573
    );
    
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.07, 0.02, 0.1)))
        .insert_resource(Grid(hg))
        .insert_resource(PlayerOwner(None))
        .add_systems(PreStartup, (
            setup::setup,
            owners::setup,
        ))
        .add_systems(Startup, (
            setup::populate_grid,
        ))
        .add_systems(Update, (
            inputs::click_province, 
            close_on_esc,
            turn_press,
        ))
        .run();
}
