use bevy::prelude::*;
use bevy::window::close_on_esc;
use rand::Rng;
use crate::owners::Owner;
use crate::systems::province_click::province_click;
use crate::systems::turn_press::turn_press;
use crate::systems::populate_grid::populate_grid;
use crate::prelude::{HexGridResource, MainCamera};

pub struct EngineSetupPlugin;

impl Plugin for EngineSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (
            setup,
            setup_o,
        ))
        .add_systems(Startup, (
            populate_grid,
        ));
    }
}

pub struct EngineUpdatePlugin;

impl Plugin for EngineUpdatePlugin {
    fn build(&self, app: &mut App) {
        app .add_systems(Update, (
            province_click,
            close_on_esc,
            turn_press,
        ));
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn setup_o(
    mut commands: Commands,
    grid: Res<HexGridResource>,
) {
    let mut rng = rand::thread_rng();
    let taken_positions = vec![];

    for _ in 0..8 {
        let pos = (
            rng.gen_range(0..grid.0.width()),
            rng.gen_range(0..grid.0.height())
        );

        if taken_positions.contains(&pos) { continue; }

        commands.spawn( Owner::new(pos, &mut rng));
    }
}
