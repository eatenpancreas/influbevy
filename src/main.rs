mod hex_grid;
#[cfg(test)]
mod tests;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::{close_on_esc, PrimaryWindow};
use rand::Rng;
use hex_grid::HexGrid;
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;

#[derive(Resource)]
struct Grid(HexGrid<Option<Entity>>);

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
        .add_systems(Update, close_on_esc)
        .insert_resource(ClearColor(Color::rgb(0.07, 0.02, 0.1)))
        .insert_resource(Grid(hg))
        .add_systems(Startup, (populate))
        .add_systems(Update, (mouse_button_inputs))
        .run();
}

fn populate(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    asset_server: Res<AssetServer>
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

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
                        color: Color::rgb(
                            rng.gen_range(0.2..1.),
                            rng.gen_range(0.2..1.),
                            rng.gen_range(0.2..1.)
                        ),
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

fn mouse_button_inputs(
    mut mouse_btn_evr: EventReader<MouseButtonInput>,
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    for ev in mouse_btn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => match ev.button {
                MouseButton::Left => {
                    let (camera, camera_transform) = q_camera.single();
                    let window = q_windows.single();
                    
                    if let Some(world_position) = window.cursor_position()
                        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                        .map(|ray| ray.origin.truncate())
                    {
                        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
                        let (x, y) = grid.0.get_pos_at_v2(world_position, 0.0);
                        let pos = grid.0.get_mut(x, y);
                        eprintln!("pos: {:?}", pos);
                        
                        if let Some(pos) = pos {
                            if let Some(entity) = pos.t {
                                commands.entity(entity).despawn();
                                pos.t = None;
                            }
                        }
                    }
                }
                MouseButton::Right => {
                    println!("Right mouse button press");
                }
                _ => {}
            },
            _ => {}
        }
    }
}