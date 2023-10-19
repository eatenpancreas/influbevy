use bevy::prelude::*;
use rand::prelude::*;
use crate::prelude::*;

#[derive(Component)]
pub struct Owner {
    pub col: Color,
    pub starting_pos: V2,
    pub positions: Vec<V2>
}

impl Owner {
    pub fn new(pos: V2, rng: &mut ThreadRng) -> Self {
        Self {
            col: random_color(rng),
            starting_pos: pos,
            positions: vec![pos]
        }
    }
}

#[derive(Resource)]
pub struct PlayerOwner<'p>(pub Option<&'p Entity>);

fn random_color(rng: &mut ThreadRng) -> Color {
    Color::rgb(rng.gen_range(0.2..0.95), rng.gen_range(0.2..0.95), rng.gen_range(0.2..0.95))
}