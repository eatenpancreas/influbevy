use bevy::prelude::Component;

pub use crate::hex_grid::*;
pub use crate::hex_grid::position::*;
pub use crate::tile::*;

pub use crate::adjacency::*;
pub use crate::v2::*;
pub use crate::owners::*;
pub use crate::core::*;

#[derive(Component)]
pub struct MainCamera;