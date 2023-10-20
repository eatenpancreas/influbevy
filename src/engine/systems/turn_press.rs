
use bevy::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::prelude::*;

pub fn turn_press(
    keyboard: Res<Input<KeyCode>>,
    mut owners_q: Query<(Entity, &mut Owner)>,
    mut sprite_q: Query<&mut Sprite>,
    mut grid: ResMut<HexGridResource>,
) {
    // if keyboard.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();
        
        let mut removals = vec![];
        for mut owner in owners_q.iter_mut() {
            // random strength increase
            for pos in &owner.1.positions {
                if rng.gen_range(0.0..1.) > 0.995 {
                    set_strength(*pos, &mut grid.0, &mut sprite_q, |s| s + 1);
                }
            }
            
            let mut newly_owned: Vec<V2> = vec![];
            for v in get_adjacent(&owner, &grid.0) {
                if do_conquest(&mut rng, v, &grid.0) {
                    set_strength(v, &mut grid.0, &mut sprite_q, |s| s - 3);
                    if let Some(tile) = grid.0.get_mut(v).and_then(|p| p.tile.as_mut()) {
                        sprite_q.get_mut(tile.inner_entity).unwrap().color = owner.1.col;
                        sprite_q.get_mut(tile.entity).unwrap().color = owner.1.col;
                        newly_owned.push(v);

                        let old_owner = std::mem::replace(&mut tile.owner, Some(owner.0));
                        if let Some(o) = old_owner { removals.push((o, v)); }
                    }
                }
            }
            
            owner.1.positions.extend(newly_owned.iter());
        }
        
        for (o, (nx, ny)) in removals {
            owners_q.get_mut(o).unwrap().1.positions.retain(|(x, y)| *x != nx || *y != ny);
        }
    // }
}

fn set_strength(pos: V2, grid: &mut HexGrid, sprite_q: &mut Query<&mut Sprite>, set_strength: impl Fn(i32) -> i32) {
    let size = grid.pos_size();
    if let Some(tile) = grid.get_mut(pos).and_then(|p| p.tile.as_mut()) {
        tile.strength = 0.max((MAX_STRENGTH as i32).min(set_strength(tile.strength as i32))) as u8;
        sprite_q.get_mut(tile.inner_entity).unwrap().custom_size = Some(size * (tile.strength as f32 * (1.0 / MAX_STRENGTH as f32)));
    }
}

fn do_conquest(rng: &mut ThreadRng, v2: V2, grid: &HexGrid) -> bool {
    let adjacent = get_adjacent_single(v2, grid);
    let adj_len = adjacent.len() as f32 * 0.01;
    let stren = calculate_strength(adjacent, grid) as f32 * 0.001;
    rng.gen_range(0.0..1.0) > 0.99 + stren - adj_len
}