use std::{
    ops::Add,
    collections::HashSet,
};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Coord2D<T: Add>(pub T, pub T);

impl<T: Add<Output=T>> Add for Coord2D<T> {
    type Output = Coord2D<T>;

    fn add(self, rhs: Coord2D<T>) -> Coord2D<T> {
        Coord2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub fn box_generator(width: i32, height: i32) -> HashSet<Coord2D<i32>> {
    let mut set: HashSet<Coord2D<i32>> = HashSet::with_capacity((2*width + 2*height) as usize);
    for i in 0..=width+1 {
        set.insert(Coord2D(i, 0));
        set.insert(Coord2D(i, height+1));
    }
    for i in 0..=height+1 {
        set.insert(Coord2D(0, i));
        set.insert(Coord2D(width+1, i));
    }
    set
}
