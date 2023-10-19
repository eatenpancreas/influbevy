mod tmp;

use bevy::prelude::*;
use engine::plugins::EnginePlugin;
use engine::plugins::tmp::TempPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnginePlugin::new(None))
        .add_plugins(TempPlugin)
        .run();
}
