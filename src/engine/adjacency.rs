use bevy::prelude::{Entity, Mut};
use crate::engine::hex_grid::HexGrid;
use crate::engine::v2::Point;
use crate::owners::Owner;
use crate::Tile;

pub type TileGrid = HexGrid<Option<Tile>>;
pub fn get_adjacent(owner: &(Entity, Mut<Owner>,), grid: &TileGrid) -> Vec<Point> {
    let mut adjacent: Vec<Point> = vec![];
    
    for (x, y) in &owner.1.positions {
        let nb = grid.get_neighbours(*x, *y);
        for neighbour in nb {
            let tile = neighbour.t.as_ref().unwrap();
            
            match tile.owner {
                None => adjacent.push((neighbour.x, neighbour.y)),
                Some(other_owner) => if other_owner != owner.0 { 
                    adjacent.push((neighbour.x, neighbour.y));
                }
            }
        }
    }
    
    adjacent
}

pub fn get_adjacent_single(x: u16, y: u16, grid: &TileGrid) -> Vec<Point> {
    let mut adjacent: Vec<Point> = vec![];
    
    if let Some(p) = grid.get(x, y) {
        let owner = p.t.as_ref().unwrap().owner;
        
        if let Some(o) = owner {

            let nb = grid.get_neighbours(x, y);
            for neighbour in nb {
                let n_tile = neighbour.t.as_ref().unwrap();

                match n_tile.owner {
                    None => adjacent.push((neighbour.x, neighbour.y)),
                    Some(no) => {
                        if no != o { adjacent.push((neighbour.x, neighbour.y)); }
                    },
                }
            }
        }
    }
    
    adjacent
}