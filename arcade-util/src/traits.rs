use super::coords::Coord2D;

pub trait Collidable<T: std::ops::Add> {
    fn collides_with(&self, coord: &Coord2D<T>) -> bool;
}

pub trait CoordConfiguration<'a, T: std::ops::Add> {
    fn configuration(&'a self) -> Box<dyn Iterator<Item = &'a Coord2D<T>> + 'a>;
}
