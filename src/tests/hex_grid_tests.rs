use bevy::math::Rect;
use crate::hex_grid::HexGrid;

#[test]
pub fn hex_grid_test() {
    let size = Rect::new(0., 0., 1., 1.);
    
    let h = HexGrid::new(2, 2, size, 0.0,vec![1, 1, 1, 1]).unwrap();
    let h1 = HexGrid::new(2, 4, size,0.0, vec![1, 1, 1, 1]);
    let h2 = HexGrid::<f32>::new(0, 0, size,0.0, vec![]);

    assert_eq!(h.get(0, 0).unwrap().t, 1);
    assert!(h.get(0, 2).is_none());
    assert_eq!(h.get(1, 1).unwrap().t, 1);

    assert!(h1.is_err());
    assert!(h2.is_err());
}

#[test]
pub fn hex_grid_test_empty() {
    let size = Rect::new(0., 0., 1., 1.);
    let mut empty = HexGrid::<u8>::new_empty(2, 3, size, 0.0,);
    
    assert!(empty.get(0, 0).unwrap().t.is_none());
    assert!(empty.get(4, 3).is_none());
    empty.set(0, 0, Some(1));
    assert_eq!(empty.get(0, 0).unwrap().t, Some(1));
    
    let i = empty.replace(0, 0, Some(2));
    assert_eq!(i.unwrap(), Some(1));
    assert_eq!(empty.get(0, 0).unwrap().t, Some(2));
}


#[test]
pub fn hex_grid_test_neigh() {
    let size = Rect::new(0., 0., 1., 1.);
    let empty = HexGrid::<u8>::new_empty(4, 3, size, 0.0,);
    let n = empty.get(1, 1).unwrap().get_neighbors(&empty);
    let n2 = empty.get(2, 0).unwrap().get_neighbors(&empty);
    let n3 = empty.get(0, 2).unwrap().get_neighbors(&empty);
    let n4 = empty.get(3, 1).unwrap().get_neighbors(&empty);
    
    assert_eq!(n.len(), 6);
    assert_eq!((n[0].x, n[0].y), (0, 1));
    assert_eq!((n[1].x, n[1].y), (2, 1));
    assert_eq!((n[2].x, n[2].y), (1, 0));
    assert_eq!((n[3].x, n[3].y), (1, 2));
    assert_eq!((n[4].x, n[4].y), (0, 0));
    assert_eq!((n[5].x, n[5].y), (2, 0));
    
    assert_eq!(n2.len(), 5);
    assert_eq!((n2[0].x, n2[0].y), (1, 0));
    assert_eq!((n2[1].x, n2[1].y), (3, 0));
    assert_eq!((n2[2].x, n2[2].y), (2, 1));
    assert_eq!((n2[3].x, n2[3].y), (1, 1));
    assert_eq!((n2[4].x, n2[4].y), (3, 1));


    assert_eq!(n3.len(), 2);
    assert_eq!((n3[0].x, n3[0].y), (1, 2));
    assert_eq!((n3[1].x, n3[1].y), (0, 1));


    assert_eq!(n4.len(), 4);
    assert_eq!((n3[0].x, n3[0].y), (1, 2));
    assert_eq!((n3[1].x, n3[1].y), (0, 1));
}