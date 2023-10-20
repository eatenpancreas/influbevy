use bevy::math::Rect;
use engine::prelude::*;


#[test]
pub fn hex_grid_test_neigh() {
    let size = Rect::new(0., 0., 1., 1.);
    let empty = HexGrid::new_empty((4, 3), size, 0.0);
    let n = empty.get_neighbours((0, 0));
    let n2 = empty.get_neighbours((2, 0));
    let n3 = empty.get_neighbours((0, 2));
    let n4 = empty.get_neighbours((3, 1));

    assert_eq!(empty.size, (4, 3));

    assert_eq!(n.len(), 2);
    assert_eq!((n[0].x(), n[0].y()), (1, 0));
    assert_eq!((n[1].x(), n[1].y()), (0, 1));

    assert_eq!(n2.len(), 3);
    assert_eq!((n2[0].x(), n2[0].y()), (1, 0));
    assert_eq!((n2[1].x(), n2[1].y()), (3, 0));
    assert_eq!((n2[2].x(), n2[2].y()), (2, 1));


    assert_eq!(n3.len(), 3);
    assert_eq!((n3[0].x(), n3[0].y()), (1, 2));
    assert_eq!((n3[1].x(), n3[1].y()), (0, 1));
    assert_eq!((n3[2].x(), n3[2].y()), (1, 1));


    assert_eq!(n4.len(), 4);
    assert_eq!((n4[0].x(), n4[0].y()), (2, 1));
    assert_eq!((n4[1].x(), n4[1].y()), (3, 0));
    assert_eq!((n4[2].x(), n4[2].y()), (3, 2));
    assert_eq!((n4[3].x(), n4[3].y()), (2, 2));
}