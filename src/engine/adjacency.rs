use bevy::prelude::*;
use crate::prelude::*;

pub fn get_adjacent(owner: &(Entity, Mut<Owner>,), grid: &HexGrid) -> Vec<V2> {
    let mut adjacent: Vec<V2> = vec![];
    
    for v in &owner.1.positions {
        let nb = grid.get_neighbours(*v);
        for neighbour in nb {
            match neighbour.tile.as_ref().and_then(|t| t.owner) {
                None => adjacent.push(neighbour.pos),
                Some(other_owner) => if other_owner != owner.0 {
                    adjacent.push(neighbour.pos);
                }
            }
        }
    }
    
    adjacent
}

pub fn calculate_strength(positions: Vec<V2>, grid: &HexGrid) -> u8 {
    let mut strength = 0;
    
    for v in positions {
        match grid.get(v).and_then(|p| p.tile.as_ref()) {
            Some(tile) => strength += tile.strength,
            None => strength += 1,
        }
    }
    
    strength
}


pub fn get_adjacent_single(v2: V2, grid: &HexGrid) -> Vec<V2> {
    let mut adjacent: Vec<V2> = vec![];
    if let Some(o) = grid.get(v2)
        .and_then(|p| p.tile.as_ref())
        .and_then(|t| t.owner) {
        
        let nb = grid.get_neighbours(v2);
        for neighbour in nb {
            match neighbour.tile.as_ref().and_then(|t| t.owner) {
                None => adjacent.push(neighbour.pos),
                Some(no) => {
                    if no != o { adjacent.push(neighbour.pos); }
                },
            }
        }
    }
    
    adjacent
}