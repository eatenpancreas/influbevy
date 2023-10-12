mod hex_grid;
#[cfg(test)]
mod tests;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use rand::Rng;
use hex_grid::HexGrid;

#[derive(Resource)]
struct Grid(HexGrid<Option<Entity>>);

fn main() {
    let hg = HexGrid::new_empty(
        20, 10, 
        Rect::new(-600.0, -300.0, 600.0, 300.0),
        0.28571428571428573
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
    mut grid: ResMut<Grid>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    let (width, height) = grid.0.size_t();
    let size = grid.0.pos_size();
    let texture = asset_server.load("sprites/hex/hex2.png");
    let rng = &mut rand::thread_rng();
        
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
                        color: Color::rgb(rng.gen(), rng.gen(), rng.gen()),
                        custom_size: Some(size),
                        ..default()
                    },
                    texture: texture.clone(),
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