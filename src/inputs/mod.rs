mod turn_press;

pub use self::turn_press::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::{Grid, MainCamera};


pub(crate) fn click_province(
    buttons: Res<Input<MouseButton>>,
    mut grid: ResMut<Grid>,
    // mut player: ResMut<PlayerOwner>,
    mut sprite_query: Query<&mut Sprite>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor) = get_cursor_pos(q_windows, q_camera) {
            let (x, y) = grid.0.get_pos_at_v2(cursor);
            let pos = grid.0.get_mut(x, y);

            if let Some(pos) = pos {
                if let Some(tile) = &pos.t {
                    sprite_query.get_mut(tile.entity).unwrap().color = Color::rgb(0.1, 0.1, 0.1);
                    sprite_query.get_mut(tile.inner_entity).unwrap().color = Color::rgb(0.1, 0.1, 0.2);
                }
            }

            for n in grid.0.get_neighbours(x, y) {
                if let Some(tile) = &n.t {
                    sprite_query.get_mut(tile.entity).unwrap().color = Color::rgb(0.1, 0.1, 0.1);
                    sprite_query.get_mut(tile.inner_entity).unwrap().color = Color::rgb(0.1, 0.1, 0.2);
                }
            }
        }
    }
}

pub fn get_cursor_pos(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = q_camera.single();
    let window = q_windows.single();
    
    window.cursor_position().and_then(
        |cursor| camera.viewport_to_world(camera_transform, cursor)
    ).map(
        |ray| ray.origin.truncate()
    )
}