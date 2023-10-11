use bevy::math::{Rect, Vec2};
use crate::hex_grid::{HexGrid, Pos};

impl <T> Pos<T> {
    pub fn phys_center<UT>(&self, hg: &HexGrid<UT>) -> Vec2 {
        self.phys_min(hg) + self.phys_size(hg) / 2.0
    }
    
    pub fn phys_min<UT>(&self, hg: &HexGrid<UT>) -> Vec2 {
        let x = self.x as f32 * hg.physical_size.width() / hg.width as f32;
        let mut y = self.y as f32 * hg.physical_size.height() / hg.height as f32;
        if self.x % 2 == 1 {
            y += hg.physical_size.height() / hg.height as f32 / 2.0;
        }

        Vec2::new(x, y)
    }
    
    pub fn phys_max<UT>(&self, hg: &HexGrid<UT>) -> Vec2 {
        self.phys_min(hg) + self.phys_size(hg)
    }

    pub fn phys_size<UT>(&self, hg: &HexGrid<UT>) -> Vec2 {
        // get size of the hexagon
        let width = hg.physical_size.width() / hg.width as f32;
        let height = hg.physical_size.height() / hg.height as f32;
        
        Vec2::new(width, height)
    }

    pub fn phys_rect<UT>(&self, hg: &HexGrid<UT>) -> Rect {
        Rect::from_corners(self.phys_min(hg), self.phys_max(hg))
    }
}