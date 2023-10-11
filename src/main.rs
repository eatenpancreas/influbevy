mod hex_grid;
#[cfg(test)]
mod tests;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use hex_grid::HexGrid;

#[derive(Resource)]
struct Grid(HexGrid<Option<Entity>>);

fn main() {
    let hg = HexGrid::new_empty(
        10, 10, 
        Rect::new(-500.0, -300.0, 500.0, 300.0)
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

    let (width, height) = grid.0.size_t();
    let size = grid.0.pos_size();
    
    for xx in 0..width {
        for yy in 0..height {
            let (x, y) = grid.0.pos_center(xx, yy).into();

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

            let pos = grid.0.get_mut(xx, yy).unwrap();
            pos.set(Some(commands.spawn(bundle).id()));
        }
    }
}

fn display() {

}