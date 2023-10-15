use crate::lib::hex_grid::HexGrid;

const U16_MAX_AS_I32: i32 = u16::MAX as i32;

#[derive(Debug)]
pub struct Pos<T> {
    pub x: u16,
    pub y: u16,
    pub t: T,
}

impl <T> Pos<T> {
    pub fn new(x: u16, y: u16, t: T) -> Pos<T> {
        Pos { x, y, t }
    }
    
    pub fn set(&mut self, t: T) {
        self.t = t;
    }
    
    pub fn get_neighbors<'a, UT>(&'a self, hg: &'a HexGrid<UT>) -> Vec<&Pos<UT>> {
        let mut neighbors = vec![];
        neighbors.extend(self.dir(-1, 0, hg));
        neighbors.extend(self.dir(1, 0, hg));
        neighbors.extend(self.dir(0, -1, hg));
        neighbors.extend(self.dir(0, 1, hg));

        if self.y % 2 == 0 {
            neighbors.extend(self.dir(-1, 1, hg));
            neighbors.extend(self.dir(1, 1, hg));
        } else {
            neighbors.extend(self.dir(-1, -1, hg));
            neighbors.extend(self.dir(1, -1, hg));
        }

        neighbors
    }

    pub fn dir<'a, UT>(&'a self, dir_x: i32, dir_y: i32, hg: &'a HexGrid<UT>) -> Option<&Pos<UT>> {
        let curr_x = self.x as i32;
        let curr_y = self.y as i32;

        if curr_x + dir_x < 0 || curr_x + dir_x >= U16_MAX_AS_I32 {
            return None
        } else if curr_y+ dir_y < 0 || curr_y + dir_y >= U16_MAX_AS_I32 {
            return None
        }

        hg.get((curr_x + dir_x) as u16, (curr_y + dir_y) as u16)
    }
}
