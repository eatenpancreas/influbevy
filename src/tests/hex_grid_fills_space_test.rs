use bevy::math::Rect;
use crate::hex_grid::HexGrid;

#[test]
pub fn hex_grid_fills_space_test_1() {
    let hg = HexGrid::<u8>::new_empty(
        5, 1,
        Rect::new(0.0, 0.0, 100.0, 1.0), 0.2,
    );
    assert_eq!(fmt_end(&hg, 4), "100.00");
}


#[test]
pub fn hex_grid_fills_space_test_2() {
    let hg = HexGrid::<u8>::new_empty(
        5, 1,
        Rect::new(132.0, 0.0, 14500.0, 1.0), 0.3,
    );
    assert_eq!(fmt_end(&hg, 4), "14500.00");
}


#[test]
pub fn hex_grid_fills_space_test_3() {
    let hg = HexGrid::<u8>::new_empty(
        5, 1,
        Rect::new(-323.0, 0.0, 1.0, 1.0), 0.4,
    );
    assert_eq!(fmt_end(&hg, 4), "1.00");
}

fn fmt_range<T>(hg: &HexGrid<T>, x: u16) -> String {
    let min = format!("{:.1$}", hg.pos_min(x, 0).x, 2);
    let max = format!("{:.1$}", hg.pos_max(x, 0).x, 2);
    format!("{}, {}", min, max)
}
fn fmt_end<T>(hg: &HexGrid<T>, x: u16) -> String {
    let max = format!("{:.1$}", hg.pos_max(x, 0).x, 2);
    format!("{}", max)
}