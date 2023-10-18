use bevy::prelude::*;
use rand::Rng;
use crate::{Grid, Tile};
use crate::lib::hex_grid::Pos;
use crate::owners::Owner;
use crate::lib::adjacency::Adjacency;

pub fn turn_press(
    keyboard: Res<Input<KeyCode>>,
    owners_q: Query<(Entity, &mut Owner)>,
    mut sprite_q: Query<&mut Sprite>,
    grid: ResMut<Grid>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        Adjacency::get_all(owners_q, grid, |adj, newly_owned| {
            let mut rng = rand::thread_rng();
            if rng.gen_range(0.1..4.0) < 3.0 {
                sprite_q.get_mut(adj.other.1.entity).unwrap().color = adj.this.0.col;
                sprite_q.get_mut(adj.other.1.inner_entity).unwrap().color = adj.this.0.col;
                
                newly_owned.push(adj);
                
                return;
            }
        })
    }
}