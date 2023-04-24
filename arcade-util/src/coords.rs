use bevy::prelude::Component;

use std::{
    ops::Add,
};

#[derive(Component, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Coord2D<T: Add>(pub T, pub T);

impl<T: Add<Output=T>> Add for Coord2D<T> {
    type Output = Coord2D<T>;

    fn add(self, rhs: Coord2D<T>) -> Coord2D<T> {
        Coord2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}
