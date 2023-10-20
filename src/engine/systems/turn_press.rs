
use bevy::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::prelude::*;
use crate::owners::Owner;

pub fn turn_press(
    keyboard: Res<Input<KeyCode>>,
    mut owners_q: Query<(Entity, &mut Owner)>,
    mut sprite_q: Query<&mut Sprite>,
    mut grid: ResMut<HexGridResource>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();
        
        let mut removals = vec![];
        for mut owner in owners_q.iter_mut() {
            let mut newly_owned: Vec<V2> = vec![];
            for v in get_adjacent(&owner, &grid.0) {
                if do_conquest(&mut rng, v, &grid.0) {
                    let adjacent = grid.0.get_mut(v).unwrap();
                    let tile = adjacent.tile.as_mut().unwrap();

                    sprite_q.get_mut(tile.inner_entity).unwrap().color = owner.1.col;
                    sprite_q.get_mut(tile.entity).unwrap().color = owner.1.col;
                    newly_owned.push(v);

                    let old_owner = std::mem::replace(&mut tile.owner, Some(owner.0));
                    if let Some(o) = old_owner { removals.push((o, v)); }
                }
            }
            
            owner.1.positions.extend(newly_owned.iter());
        }
        
        for (o, (nx, ny)) in removals {
            owners_q.get_mut(o).unwrap().1.positions.retain(|(x, y)| *x != nx || *y != ny);
        }
    }
}

fn do_conquest(rng: &mut ThreadRng, v2: V2, grid: &HexGrid) -> bool {
    rng.gen_range(0.0..1.0) > 0.97 - get_adjacent_single(v2, grid).len() as f32 * 0.02
}