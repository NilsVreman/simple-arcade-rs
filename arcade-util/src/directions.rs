use super::coords::Coord2D;

// Directions in 2D
#[derive(Copy, Clone, PartialEq)]
pub enum Dir2D {
    Up, Down, Left, Right,
}

// Rotations 2D
#[derive(Copy, Clone, PartialEq)]
pub enum Rot2D {
    Clockwise, CounterClockwise
}

impl Dir2D {
    pub fn rotate(&self, rotation: &Rot2D) -> Dir2D {
        match (self, rotation) {
            (Dir2D::Up,    Rot2D::Clockwise) | (Dir2D::Down,  Rot2D::CounterClockwise) => Dir2D::Right,
            (Dir2D::Right, Rot2D::Clockwise) | (Dir2D::Left,  Rot2D::CounterClockwise) => Dir2D::Down,
            (Dir2D::Down,  Rot2D::Clockwise) | (Dir2D::Up,    Rot2D::CounterClockwise) => Dir2D::Left,
            (Dir2D::Left,  Rot2D::Clockwise) | (Dir2D::Right, Rot2D::CounterClockwise) => Dir2D::Up,
        }
    }

    pub fn as_coord(&self) -> Coord2D<i32> {
        match self {
            Dir2D::Up => Coord2D(0, 1),
            Dir2D::Down => Coord2D(0, -1),
            Dir2D::Left => Coord2D(-1, 0),
            Dir2D::Right => Coord2D(1, 0),
        }
    }
}
