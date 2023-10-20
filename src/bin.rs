mod tmp;
#[cfg(test)]
mod tests;

use bevy::prelude::*;
use engine::plugins::EnginePlugin;
use engine::plugins::tmp::EngineSetupPlugin;
use engine::plugins::tmp::EngineUpdatePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnginePlugin::new(None))
        .add_plugins((EngineSetupPlugin, EngineUpdatePlugin))
        .run();
}
