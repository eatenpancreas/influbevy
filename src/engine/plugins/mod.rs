mod engine_setup_plugin;

pub use self::engine_setup_plugin::{EngineSetupPlugin, EngineUpdatePlugin};

use bevy::prelude::*;
use crate::prelude::*;

pub struct EnginePlugin {
    config: EngineConfig
}

pub struct EngineConfig {
    size: V2,
}

impl EnginePlugin {
    pub fn new(config: Option<EngineConfig>) -> Self {
        let config = config.unwrap_or(EngineConfig {
            size: (60, 30),
        });
        
        Self { config }
    }
}

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        let grid = HexGrid::new_empty(
            self.config.size,
            Rect::new(-550.0, -250.0, 550.0, 250.0),
            0.28571428571428573
        );
        let clear_color = Color::rgb(0.1, 0.1, 0.1);

        app .insert_resource(ClearColor(clear_color))
            .insert_resource(HexGridResource(grid))
            .insert_resource(PlayerOwner(None));
    }
}
