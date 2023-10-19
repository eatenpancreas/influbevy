mod pos_phys;
mod pos;

pub use self::pos::*;
use bevy::math::{Rect};

const U16_MAX_AS_I32: i32 = u16::MAX as i32;

#[derive(Debug)]
pub struct HexGrid<T> {
    pub fields: Vec<Pos<T>>,
    pub height: u16,
    pub width: u16,
    pub physical_size: Rect,
    pub hex_slope_pct: f32,
}

impl <T> HexGrid<T> {
    pub fn new_empty(width: i16, height: i16, physical_size: Rect, hex_slope_pct: f32) -> HexGrid<Option<T>> {
        let mut grid = HexGrid { fields: vec![], width: width as u16, height: height as u16, physical_size, hex_slope_pct };
        for y in 0..height {
            for x in 0..width {
                grid.fields.push(
                    Pos::new(x as u16, y as u16, None)
                );
            }
        }
        grid
    }
    
    pub fn get(&self, x: u16, y: u16) -> Option<&Pos<T>> {
        if x >= self.width || y >= self.height { return None }
        let i = x + y * self.width;
        self.fields.get(i as usize)
    }
    
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Pos<T>> {
        if x >= self.width || y >= self.height { return None }
        let i = x + y * self.width;
        self.fields.get_mut(i as usize)
    }
    
    pub fn get_neighbours(&self, x: u16, y: u16) -> Vec<&Pos<T>> {
        let mut neighbors = vec![];
        neighbors.extend(self.dir(-1, 0, x, y));
        neighbors.extend(self.dir(1, 0, x, y));
        neighbors.extend(self.dir(0, -1, x, y));
        neighbors.extend(self.dir(0, 1, x, y));

        if x % 2 == 1 {
            neighbors.extend(self.dir(-1, 1, x, y));
            neighbors.extend(self.dir(1, 1, x, y));
        } else {
            neighbors.extend(self.dir(-1, -1, x, y));
            neighbors.extend(self.dir(1, -1, x, y));
        }

        neighbors
    }

    fn dir(&self, dir_x: i32, dir_y: i32, x: u16, y: u16) -> Option<&Pos<T>> {
        let curr_x = x as i32;
        let curr_y = y as i32;

        if curr_x + dir_x < 0 || curr_x + dir_x >= U16_MAX_AS_I32 {
            return None
        } else if curr_y+ dir_y < 0 || curr_y + dir_y >= U16_MAX_AS_I32 {
            return None
        }

        self.get((curr_x + dir_x) as u16, (curr_y + dir_y) as u16)
    }
    pub fn size_t(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}