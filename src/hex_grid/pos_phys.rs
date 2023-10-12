use bevy::math::{Rect, Vec2};
use crate::hex_grid::{HexGrid};

impl <T> HexGrid<T> {
    pub fn pos_center(&self, x: u16, y: u16) -> Vec2 {
        self.pos_min(x,y) + self.pos_size() / 2.0
    }
    
    fn _hex_height(&self) -> f32 {
        self.physical_size.height() / self.height as f32
    }
    fn _hex_width(&self) -> f32 {
        (self.physical_size.width() * (1.0 + self.hex_slope_pct)) / self.width as f32
    }
    
    pub fn pos_min(&self, in_x: u16, in_y: u16) -> Vec2 {
        let x = in_x as f32 * (self._hex_width() * (1.0 - self.hex_slope_pct)) + self.physical_size.min.x;
        let mut y = in_y as f32 * self._hex_height() + self.physical_size.min.y;
        if in_x % 2 == 1 {
            y += self.physical_size.height() / self.height as f32 / 2.0;
        }

        Vec2::new(x, y)
    }
    
    pub fn pos_max(&self, x: u16, y: u16) -> Vec2 {
        self.pos_min(x,y) + self.pos_size()
    }

    pub fn pos_size(&self) -> Vec2 {
        // get size of the hexagon
        
        Vec2::new(self._hex_width(), self._hex_height())
    }

    pub fn pos_rect(&self, x: u16, y: u16) -> Rect {
        Rect::from_corners(self.pos_min(x,y), self.pos_max(x,y))
    }
}