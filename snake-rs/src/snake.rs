use std::collections::{HashSet, VecDeque};

use arcade_util::{
    Coord2D,
    Dir2D,
    Rot2D,
    Collidable,
    CoordConfiguration,
};

pub struct Snake {
    pub segments: Vec<Coord2D<i32>>,
    direction: Dir2D,
    segments_to_add: VecDeque<Coord2D<i32>>,
}

impl Snake {
    pub fn new(start_pos: Coord2D<i32>) -> Self {
        Self {
            segments: vec![start_pos],
            direction: Dir2D::Up,
            segments_to_add: VecDeque::with_capacity(5),
        }
    }

    pub fn move_forward(&mut self) -> Coord2D<i32> {
        let new_head = self.segments[0] + self.direction.as_coord();
        self.segments.insert(0, new_head);
        // If we have elements to add and the currently popped one was the one to add.
        if let Some(last) = self.segments.pop() {
            if !self.segments_to_add.is_empty() && last == self.segments_to_add[0] {
                self.segments.push(last);
                self.segments_to_add.pop_front().unwrap();
            }
        }
        new_head
    }

    pub fn rotate(&mut self, rot: &Rot2D) {
        self.direction = self.direction.rotate(&rot);
    }

    pub fn grow(&mut self) {
        self.segments_to_add.push_back(self.segments[0]);
    }

    pub fn head_coord(&self) -> Coord2D<i32> {
        self.segments[0]
    }

    pub fn collides_with_self(&self) -> bool {
        self.segments[1..].iter().any(|&c| c == self.segments[0])
    }
}

impl Collidable<i32> for Snake {
    /// NOTE: This should be enough to check head
    fn collides_with(&self, coord: &Coord2D<i32>) -> bool {
        &self.segments[0] == coord
    }
}

impl CoordConfiguration<i32> for Snake {
    fn configuration(&self) -> HashSet<Coord2D<i32>> {
        self.segments.clone().into_iter().collect()
    }
}
