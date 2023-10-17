use bevy::math::vec3;
use bevy::prelude::*;
use crate::{Grid, Tile};
use crate::owners::Owner;

pub(crate) fn populate_grid(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    asset_server: Res<AssetServer>,
    owners: Query<(Entity, &Owner)>
) {
    let (width, height) = grid.0.size_t();
    let size = grid.0.pos_size();
    let texture = asset_server.load("sprites/hex/hex1.png");
    let texture2 = asset_server.load("sprites/hex/hex2.png");

    for xx in 0..width {
        for yy in 0..height {
            let (x, y) = grid.0.pos_center(xx, yy).into();
            let (entity_owner, owner) = match owners.iter().find(|(_, o)| o.starting_pos == (xx, yy)) {
                None => (None, None),
                Some((e, o)) => (Some(e), Some(o))
            };
            
            
            let color = match owner {
                None => Color::rgb(0.1, 0.1, 0.1),
                Some(o) => o.col
            };
            
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
                owner: entity_owner,
            }));
        }
    }
}
