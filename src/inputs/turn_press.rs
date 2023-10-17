use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::de::Unexpected::Option;
use rand::Rng;
use crate::{Grid};
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

        get_conq(owners_q, grid, |this, other| {
            
        })
        
        // for mut owner in owners_q.iter_mut() {
        //     let mut newly_owned = vec![];
        //     
        //     for (x, y) in &owner.positions {
        //         if let Some(pt) = grid.0.get(*x, *y) {
        //             let neighbours = pt.get_neighbors(&grid.0);
        //             let len = neighbours.len();
        //             
        //             for neighbour in neighbours {
        //                 if !(rng.gen_range(0..len * 2) < 1) { continue; }
        //                 if let Some(n) = &neighbour.t {
        //                     if let Some(o) = n.owner {
        //                         if let Some(t) = &pt.t {
        //                             if let Some(e) = t.owner {
        //                                 if e == o { continue; }
        //                             }
        //                         }
        //                         
        //                         removals.push((o, (neighbour.x, neighbour.y)));
        // 
        //                         if rng.gen_range(0.1..4.0) < 3.0 {
        //                             sprite_q.get_mut(n.inner_entity).unwrap().color = Color::rgb(0.1, 0.1, 0.1);
        //                             sprite_q.get_mut(n.entity).unwrap().color = Color::rgb(0.1, 0.1, 0.1);
        //                             
        //                             continue;
        //                         }
        //                     }
        //                     newly_owned.push((neighbour.x, neighbour.y));
        //                     
        //                     sprite_q.get_mut(n.inner_entity).unwrap().color = owner.col;
        //                     sprite_q.get_mut(n.entity).unwrap().color = owner.col;
        //                 }
        //             }
        //         }
        //     }
        // 
        //     owner.positions.extend(newly_owned.iter());
        //     owner.positions.sort_unstable_by(|aa, bb| (aa.0 + aa.1 * grid.0.width).cmp(&(bb.0 + bb.1 * grid.0.width)));
        //     owner.positions.dedup_by(|aa, bb| aa.0 == bb.0 && aa.1 == bb.1);
        // }
        // 
        // for (o, (nx, ny)) in removals {
        //     owners_q.get_mut(o).unwrap().positions.retain(|(x, y)| *x != nx || *y != ny);
        // }
    }
}

pub fn get_conq(
    mut owners_q: Query<(Entity, &mut Owner)>,
    mut grid: ResMut<Grid>,
    on_overlap: impl Fn(&mut Owner, &mut Owner)
) {
    let mut combinations = owners_q.iter_combinations_mut();
    while let Some([(this_owner_e, mut owner), mut other_owner]) = combinations.fetch_next() {
        for owner_position in owner.positions.iter() {
            for neighbour in grid.0.get_neighbours(owner_position.0, owner_position.1) {
                if let Some(tile) = &grid.0.get(neighbour.x, neighbour.y).unwrap().t {
                    match tile.owner {
                        None => owner.positions.remove(1),
                        Some(other_owner_e) => {
                            if other_owner_e == this_owner_e { continue; }
                            if other_owner_e == other_owner.0 {
                                on_overlap(&mut owner, &mut other_owner.1);
                            }
                        }
                    }
                }
            }
        }
    }
}