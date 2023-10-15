
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::{Grid, MainCamera};

pub(crate) fn button_inputs(
    buttons: Res<Input<MouseButton>>,
    mut grid: ResMut<Grid>,
    mut sprite_query: Query<&mut Sprite>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_windows.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            eprintln!("World coords: {}/{}", world_position.x, world_position.y);
            let (x, y) = grid.0.get_pos_at_v2(world_position, 0.0);
            let pos = grid.0.get_mut(x, y);

            if let Some(pos) = pos {
                if let Some(tile) = &pos.t {
                    sprite_query.get_mut(tile.entity).unwrap().color = Color::rgb(0.2, 0.2, 0.2);
                    sprite_query.get_mut(tile.inner_entity).unwrap().color = Color::rgb(0.2, 0.2, 0.2);
                }
            }
        }
    }
}