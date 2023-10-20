
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::prelude::*;


pub(crate) fn click_province(
    buttons: Res<Input<MouseButton>>,
    mut grid: ResMut<HexGridResource>,
    // mut player: ResMut<PlayerOwner>,
    mut sprite_query: Query<&mut Sprite>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor) = get_cursor_pos(q_windows, q_camera) {
            let v = grid.0.get_v2_at_pos(cursor);
            let pos = grid.0.get_mut(v);

            if let Some(pos) = pos { set_col(&pos.tile, &mut sprite_query); }
            for n in grid.0.get_neighbours(v) { set_col(&n.tile, &mut sprite_query) }
        }
    }
}

fn set_col(tile: &Option<Tile>, mut sprite_query: &mut Query<&mut Sprite>) {
    if let Some(tile) = tile {
        sprite_query.get_mut(tile.entity).unwrap().color = Color::rgb(0.1, 0.1, 0.1);
        sprite_query.get_mut(tile.inner_entity).unwrap().color = Color::rgb(0.1, 0.1, 0.2);
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