mod hex_grid;
#[cfg(test)]
mod tests;

use bevy::math::vec3;
//#![windows_subsystem = "windows"]
use bevy::prelude::*;
use bevy::window::close_on_esc;
use hex_grid::HexGrid;

#[derive(Resource)]
struct Grid(HexGrid<Option<Entity>>);

fn main() {
    let hg = HexGrid::new_empty(
        10, 10, 
        Rect::new(0.0, 0.0, 1000.0, 1000.0)
    );
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, close_on_esc)
        .insert_resource(Grid(hg))
        .add_systems(Startup, (populate, display.after(populate)))
        .run();
}

fn populate(
    mut commands: Commands,
    mut grid: ResMut<Grid>
) {
    commands.spawn(Camera2dBundle::default());
    
    for mut pos in grid.0.iter() {
        let (x, y) = pos.phys_center(&grid.0).into();
        let size = pos.phys_size(&grid.0);
        
        let bundle = (
            SpriteBundle {
                transform: Transform {
                    translation: vec3(x, y, 0.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.3, 0.1, 0.3),
                    custom_size: Some(size),
                    ..default()
                },
                ..default()
            },
        );
        
        pos.set(Some(commands.spawn(bundle).id()));
    }
}

fn display() {

}