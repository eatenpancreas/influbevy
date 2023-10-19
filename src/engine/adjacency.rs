use bevy::prelude::*;
use crate::prelude::*;

pub fn get_adjacent(owner: &(Entity, Mut<Owner>,), grid: &HexGrid) -> Vec<V2> {
    let mut adjacent: Vec<V2> = vec![];
    
    for (x, y) in &owner.1.positions {
        let nb = grid.get_neighbours(*x, *y);
        for neighbour in nb {
            let tile = neighbour.tile.as_ref().unwrap();
            
            match tile.owner {
                None => adjacent.push(neighbour.pos),
                Some(other_owner) => if other_owner != owner.0 { 
                    adjacent.push(neighbour.pos);
                }
            }
        }
    }
    
    adjacent
}

pub fn get_adjacent_single(x: u16, y: u16, grid: &HexGrid) -> Vec<V2> {
    let mut adjacent: Vec<V2> = vec![];
    
    if let Some(p) = grid.get(x, y) {
        let owner = p.tile.as_ref().unwrap().owner;
        
        if let Some(o) = owner {

            let nb = grid.get_neighbours(x, y);
            for neighbour in nb {
                let n_tile = neighbour.tile.as_ref().unwrap();

                match n_tile.owner {
                    None => adjacent.push(neighbour.pos),
                    Some(no) => {
                        if no != o { adjacent.push(neighbour.pos); }
                    },
                }
            }
        }
    }
    
    adjacent
}