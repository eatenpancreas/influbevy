use bevy::prelude::Entity;

pub struct Tile {
    pub entity: Entity,
    pub inner_entity: Entity,
    pub owner: Option<Entity>,
}