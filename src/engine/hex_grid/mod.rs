mod world_math;
pub mod position;

use bevy::prelude::*;
use crate::prelude::*;

const U16_MAX_AS_I32: i32 = u16::MAX as i32;

#[derive(Resource)]
pub struct HexGridResource(pub HexGrid);

#[derive(Debug)]
pub struct HexGrid {
    pub fields: Vec<Position>,
    pub size: V2,
    pub physical_size: Rect,
    pub hex_slope_pct: f32,
}

impl HexGrid {
    pub fn new_empty(size: V2, physical_size: Rect, hex_slope_pct: f32) -> HexGrid {
        let mut grid = HexGrid { fields: vec![], size, physical_size, hex_slope_pct };
        for y in 0..size.1 {
            for x in 0..size.0 {
                grid.fields.push(
                    Position::new((x as u16, y as u16), None)
                );
            }
        }
        grid
    }
    
    pub fn width(&self) -> u16 {
        self.size.0
    }
    
    pub fn height(&self) -> u16 {
        self.size.1
    }
    
    pub fn get(&self, v2: V2) -> Option<&Position> {
        if v2.0 >= self.width() || v2.1 >= self.height() { return None }
        let i = v2.0 + v2.1 * self.width();
        self.fields.get(i as usize)
    }
    
    pub fn get_mut(&mut self, v2: V2) -> Option<&mut Position> {
        if v2.0 >= self.width() || v2.1 >= self.height() { return None }
        let i = v2.0 + v2.1 * self.width();
        self.fields.get_mut(i as usize)
    }
    
    pub fn get_neighbours(&self, v2: V2) -> Vec<&Position> {
        let mut neighbors = vec![];
        if let None = self.get(v2) { return neighbors }
        
        neighbors.extend(self.dir(-1, 0, v2));
        neighbors.extend(self.dir(1, 0, v2));
        neighbors.extend(self.dir(0, -1, v2));
        neighbors.extend(self.dir(0, 1, v2));

        if v2.0 % 2 == 1 {
            neighbors.extend(self.dir(-1, 1, v2));
            neighbors.extend(self.dir(1, 1, v2));
        } else {
            neighbors.extend(self.dir(-1, -1, v2));
            neighbors.extend(self.dir(1, -1, v2));
        }

        neighbors
    }

    fn dir(&self, dir_x: i32, dir_y: i32, pos: V2) -> Option<&Position> {
        let curr_x = pos.0 as i32;
        let curr_y = pos.1 as i32;

        if curr_x + dir_x < 0 || curr_x + dir_x >= U16_MAX_AS_I32 {
            return None
        } else if curr_y+ dir_y < 0 || curr_y + dir_y >= U16_MAX_AS_I32 {
            return None
        }

        self.get(((curr_x + dir_x) as u16, (curr_y + dir_y) as u16))
    }
}