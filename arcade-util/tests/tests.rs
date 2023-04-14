use arcade_util::Coord2D;

#[test]
fn test_coord_addition() {
    let a = Coord2D(1, 2);
    let b = Coord2D(3, 4);
    let c = a + b;
    assert_eq!(c.0, 4);
    assert_eq!(c.1, 6);
}

#[test]
fn test_coord_addition_floats() {
    let a = Coord2D(1.0, 2.0);
    let b = Coord2D(3.0, 4.0);
    let c = a + b;
    assert_eq!(c.0, 4.0);
    assert_eq!(c.1, 6.0);
}
