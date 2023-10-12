use bevy::math::Rect;
use crate::hex_grid::HexGrid;

#[test]
pub fn hex_grid_pos_test() {
    let hg = HexGrid::<u8>::new_empty(
        10, 10,
        Rect::new(-500.0, -300.0, 500.0, 300.0), 0.0,
    );


    let (x, y) = hg.pos_size().into();
    assert_eq!(x, 100.0);
    assert_eq!(y, 60.0);

    let (x, y) = hg.pos_min(0, 0).into();
    assert_eq!(x, -500.0);
    assert_eq!(y, -300.0);
    
    let (x, y) = hg.pos_center(0, 0).into();
    assert_eq!(x, -450.0);
    assert_eq!(y, -270.0);
}


#[test]
pub fn hex_grid_pos_test2() {
    let hg = HexGrid::<u8>::new_empty(
        20, 10,
        Rect::new(-600.0, -300.0, 600.0, 300.0), 0.28571428571428573,
    );

    let (x, y) = hg.pos_min(0, 0).into();
    assert_eq!(x, -600.0);
    assert_eq!(y, -300.0);

    let (x, y) = hg.pos_size().into();
    // assert_eq!(x, 77.14286);
    // assert_eq!(y, 60.0);

    let (x, y) = hg.pos_max(19, 9).into();
    assert_eq!(x, 600.0);
    assert_eq!(y, 330.0);
}


