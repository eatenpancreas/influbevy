use bevy::prelude::*;
use crate::{Grid, Tile};
use crate::lib::hex_grid::Pos;
use crate::owners::Owner;

pub struct Adjacent {
    pub pos: Pos<Option<Tile>>,
}

pub struct Adjacency<'a> {
    pub this: Adjacent,
    pub other: Adjacent
}

impl Adjacency<'_> {
    pub fn get_all(
        mut owners_q: Query<(Entity, &mut Owner)>,
        mut grid: ResMut<Grid>,
        mut on_conq: impl FnMut(Adjacency, &mut Vec<Adjacency>)
    ) {
        let mut newly_owned: Vec<Adjacency> = vec![];
        let mut combinations = owners_q.iter_combinations_mut();

        while let Some([(this_owner_e, mut owner), mut other_owner]) = combinations.fetch_next() {
            let owner_positions = owner.positions.clone();

            for owner_position in owner_positions.iter() {
                if let Some(owner_tile) = get_tile(grid.0.get_mut(owner_position.0, owner_position.1)) {
                    for neighbour in grid.0.get_neighbours(owner_position.0, owner_position.1) {
                        if let Some(other_tile) = &mut grid.0.get_mut(neighbour.x, neighbour.y).unwrap().t {
                            match other_tile.owner {
                                None => {
                                    on_conq(
                                        Adjacency {
                                            this: (&mut owner, owner_tile),
                                            this_pos: (owner_position.0, owner_position.1),
                                            this_entity: this_owner_e,
                                            other: (None, other_tile),
                                            other_entity: other_owner.0,
                                            other_pos: (neighbour.x, neighbour.y)
                                        }, &mut newly_owned
                                    );
                                }, //owner.positions.remove(1);
                                Some(other_owner_e) => {
                                    if other_owner_e == this_owner_e { continue; }
                                    if other_owner_e == other_owner.0 {
                                        on_conq(
                                            Adjacency {
                                                this: (&mut owner, owner_tile),
                                                this_pos: (owner_position.0, owner_position.1),
                                                this_entity: this_owner_e,
                                                other: (Some(&mut other_owner.1), other_tile),
                                                other_entity: other_owner.0,
                                                other_pos: (neighbour.x, neighbour.y)
                                            }, &mut newly_owned
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        newly_owned.iter_mut().for_each(|adj| {
            let (this, this_t) = &mut adj.this;
            let (other, other_t) = &mut adj.other;

            if let Some(other) = other {
                other.positions.retain(|(x, y)| *x != adj.other_pos.0 || *y != adj.other_pos.1);
            }

            other_t.owner = Some(adj.this_entity);

            this.positions.push(adj.other_pos);
            this.positions.sort_unstable_by(|aa, bb| (aa.0 + aa.1 * grid.0.width).cmp(&(bb.0 + bb.1 * grid.0.width)));
            this.positions.dedup_by(|aa, bb| aa.0 == bb.0 && aa.1 == bb.1);
        });
    }
}

fn get_tile(pos: Option<&mut Pos<Option<Tile>>>) -> Option<&mut Tile> {
    if let Some(pos) = pos {
        if let Some(tile) = &mut pos.t {
            return Some(tile)
        }
    }
    None
}