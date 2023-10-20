use bevy::prelude::Entity;

#[derive(Debug)]
pub struct Tile {
    pub entity: Entity,
    pub inner_entity: Entity,
    pub owner: Option<Entity>,
    pub strength: u8,
}