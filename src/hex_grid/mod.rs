mod pos_phys;
mod pos;

pub use self::pos::*;
pub use self::pos_phys::*;

use std::io::{Error, ErrorKind};
use bevy::math::{Rect, Vec2};


#[derive(Debug)]
pub struct HexGrid<T> {
    fields: Vec<Pos<T>>,
    pub height: u16,
    pub width: u16,
    pub physical_size: Rect,
}

impl <T> HexGrid<T> {
    pub fn new(width: u16, height: u16, physical_size: Rect, mut old_fields: Vec<T>) -> Result<HexGrid<T>, Error> {
        if width == 0 || height == 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput, 
                "The width and height of the grid must be greater than 0")
            )
        }
        if (width * height) as usize != old_fields.len() {
            return Err(Error::new(
                ErrorKind::InvalidInput, 
                "The size of the grid wasn't equal to the vector given")
            )
        }

        let mut i = 0;
        let mut grid = HexGrid { fields: vec![], width, height, physical_size };
        while let Some(t) = old_fields.pop() {
            let x = i % width;
            let y = i / width;
            
            grid.fields.push(
                Pos {
                    x, y, t,
                }
            );
            
            i += 1;
        }
        
        Ok(grid)
    }
    
    pub fn new_empty(width: i16, height: i16, physical_size: Rect) -> HexGrid<Option<T>> {
        let mut grid = HexGrid { fields: vec![], width: width as u16, height: height as u16, physical_size };
        for y in 0..height {
            for x in 0..width {
                grid.fields.push(
                    Pos {
                        x: x as u16,
                        y: y as u16,
                        t: None,
                    }
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
    
    pub fn set(&mut self, x: u16, y: u16, t: T) {
        let i = x + y * self.width;
        self.fields[i as usize].t = t;
    }
    
    pub fn iter(&mut self) -> std::slice::IterMut<Pos<T>> {
        self.fields.iter_mut()
    }
    
    pub fn replace(&mut self, x: u16, y: u16, t: T) -> Option<T> {
        let i = x + y * self.width;
        let pos = self.fields.get_mut(i as usize);
        return match pos {
            None => None,
            Some(pos) => Some(std::mem::replace(&mut pos.t, t))
        }
    }
    
    pub fn resize(&mut self, new_size: Rect) {
        self.physical_size = new_size;
    }
}