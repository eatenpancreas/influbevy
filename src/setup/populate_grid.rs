use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;
use crate::{Grid, Tile};

pub(crate) fn populate_grid(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    asset_server: Res<AssetServer>
) {
    let (width, height) = grid.0.size_t();
    let size = grid.0.pos_size();
    let texture = asset_server.load("sprites/hex/hex1.png");
    let texture2 = asset_server.load("sprites/hex/hex2.png");
    let rng = &mut rand::thread_rng();

    for xx in 0..width {
        for yy in 0..height {
            let (x, y) = grid.0.pos_center(xx, yy).into();
            let color = Color::rgb(
                rng.gen_range(0.2..1.),
                rng.gen_range(0.2..1.),
                rng.gen_range(0.2..1.)
            );

            let bundle = (
                SpriteBundle {
                    transform: Transform {
                        translation: vec3(x, y, 0.),
                        ..default()
                    },
                    sprite: Sprite {
                        color,
                        custom_size: Some(size),
                        ..default()
                    },
                    texture: texture2.clone(),
                    ..default()
                },
            );
            
            let bundle2 = (
                SpriteBundle {
                    transform: Transform { translation: vec3(x, y, 0.), ..default() },
                    sprite: Sprite { color, custom_size: Some(size / 2.), ..default() },
                    texture: texture.clone(),
                    ..default()
                },
            );

            let pos = grid.0.get_mut(xx, yy).unwrap();
            pos.set(Some(Tile {
                entity: commands.spawn(bundle).id(),
                inner_entity: commands.spawn(bundle2).id(),
                owner: None,
            }));
        }
    }
}
