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
        
        for mut owner in owners_q.iter_mut() {
            let mut newly_owned = vec![];
            
            for (x, y) in &owner.1.positions {
                if let Some(pt) = grid.0.get(*x, *y) {
                    let neighbours = pt.get_neighbors(&grid.0);
                    let len = neighbours.len();
                    
                    for neighbour in neighbours {
                        if !(rng.gen_range(0..len * 2) < 1) { continue; }
                        if let Some(n) = &neighbour.t {
                            if let Some(o) = n.owner {
                                if let Some(t) = &pt.t {
                                    if let Some(e) = t.owner {
                                        if e == o { continue; }
                                    }
                                }
                                
                                removals.push((o, (neighbour.x, neighbour.y)));
        
                                if rng.gen_range(0.1..4.0) < 3.0 {
                                    sprite_q.get_mut(n.inner_entity).unwrap().color = Color::rgb(0.1, 0.1, 0.1);
                                    sprite_q.get_mut(n.entity).unwrap().color = Color::rgb(0.1, 0.1, 0.1);
                                    
                                    continue;
                                }
                            }
                            newly_owned.push((neighbour.x, neighbour.y));
                            
                            sprite_q.get_mut(n.inner_entity).unwrap().color = owner.1.col;
                            sprite_q.get_mut(n.entity).unwrap().color = owner.1.col;
                        }
                    }
                }
            }
        
            owner.1.positions.extend(newly_owned.iter());
            owner.1.positions.sort_unstable_by(|aa, bb| (aa.0 + aa.1 * grid.0.width).cmp(&(bb.0 + bb.1 * grid.0.width)));
            owner.1.positions.dedup_by(|aa, bb| aa.0 == bb.0 && aa.1 == bb.1);
        }
        
        for (o, (nx, ny)) in removals {
            owners_q.get_mut(o).unwrap().1.positions.retain(|(x, y)| *x != nx || *y != ny);
        }
    }
}