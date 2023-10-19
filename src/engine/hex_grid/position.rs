use crate::prelude::*;

#[derive(Debug)]
pub struct Position {
    pub pos: V2,
    pub tile: Option<Tile>,
}

impl Position {
    pub fn new(pos: V2, tile: Option<Tile>) -> Position {
        Position { pos, tile }
    }
    
    pub fn x(&self) -> u16 {
        self.pos.0
    }
    pub fn y(&self) -> u16 {
        self.pos.1
    }
}
