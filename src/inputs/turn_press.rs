use bevy::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::{Grid};
use crate::engine::adjacency::{get_adjacent, get_adjacent_single, TileGrid};
use crate::engine::v2::Point;
use crate::owners::Owner;

pub fn turn_press(
    keyboard: Res<Input<KeyCode>>,
    mut owners_q: Query<(Entity, &mut Owner)>,
    mut sprite_q: Query<&mut Sprite>,
    mut grid: ResMut<Grid>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();
        
        let mut removals = vec![];
        for mut owner in owners_q.iter_mut() {
            let mut newly_owned: Vec<Point> = vec![];
            for (x, y) in get_adjacent(&owner, &grid.0) {
                let do_conq = do_conquest(&mut rng, x, y, &grid.0);
                
                if do_conq {
                    let adjacent = grid.0.get_mut(x, y).unwrap();
                    let tile = adjacent.t.as_mut().unwrap();
                    
                    sprite_q.get_mut(tile.inner_entity).unwrap().color = owner.1.col;
                    sprite_q.get_mut(tile.entity).unwrap().color = owner.1.col;
                    newly_owned.push((x, y));
                    
                    let old_owner = std::mem::replace(&mut tile.owner, Some(owner.0));
                    if let Some(o) = old_owner { removals.push((o, (x, y))); }
                }
            }
            
            owner.1.positions.extend(newly_owned.iter());
        }
        
        for (o, (nx, ny)) in removals {
            owners_q.get_mut(o).unwrap().1.positions.retain(|(x, y)| *x != nx || *y != ny);
        }
    }
}

fn do_conquest(rng: &mut ThreadRng, x: u16, y:u16, grid: &TileGrid) -> bool {
    rng.gen_range(0.0..1.0) > 0.97 - get_adjacent_single(x, y, grid).len() as f32 * 0.02
}