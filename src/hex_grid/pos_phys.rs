use crate::hex_grid::{HexGrid, Pos};

impl <T> Pos<T> {
    fn display<UT>(&self, hg: HexGrid<UT>) -> String {
        let x = hg.physical_size.x / self.x;
        let y = hg.physical_size.y / self.y;
        format!("({}, {})", self.x, self.y)
    }
}