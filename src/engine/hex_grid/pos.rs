
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
    
    // pub fn get_neighbours<'a, UT>(&'a self, hg: &'a HexGrid<UT>) -> Vec<&Pos<UT>> {
    //     hg.get_neighbours(self.x, self.y)
    // }
}
