use super::coords::Coord2D;

pub trait Collidable<T: std::ops::Add> {
    fn collides_with(&self, coord: &Coord2D<T>) -> bool;
}

pub trait CoordConfiguration<T: std::ops::Add> {
    fn configuration(&self) -> std::collections::HashSet<Coord2D<T>>;
}
