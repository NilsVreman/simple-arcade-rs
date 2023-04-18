use std::collections::HashSet;

use arcade_util::{
    Collidable,
    CoordConfiguration,
    Coord2D,
    box_generator,
};

pub struct Board {
    height: i32,
    width: i32,
    boundary: HashSet<Coord2D<i32>>,
    food_gen: FoodGenerator,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        let boundary = box_generator(width, height);
        Self {
            width,
            height,
            boundary,
            food_gen: FoodGenerator::new(height, width),
        }
    }

    pub fn food_coord(&self) -> Coord2D<i32> {
        return self.food_gen.food_coord()
    }

    pub fn new_food(&mut self) -> Option<Coord2D<i32>> {
        return self.food_gen.next()
    }

    pub fn get_width(&self) -> i32 {
        return self.width
    }

    pub fn get_height(&self) -> i32 {
        return self.height
    }
}

impl Collidable<i32> for Board {
    fn collides_with(&self, coord: &Coord2D<i32>) -> bool {
        return coord.0 < 0
            || coord.0 > self.width
            || coord.1 < 0
            || coord.1 > self.height;
    }
}

impl CoordConfiguration<i32> for Board {
    fn configuration(&self) -> HashSet<Coord2D<i32>> {
        self.boundary.clone()
    }
}

// Food Generator

struct FoodGenerator {
    width: i32,
    height: i32,
    food: Coord2D<i32>,
}

impl FoodGenerator {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            food: Coord2D(15, 16),
        }
    }

    fn food_coord(&self) -> Coord2D<i32> {
        return self.food
    }
}

impl Iterator for FoodGenerator {
    type Item = Coord2D<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_food = Coord2D(0, 2) + self.food;
        let ret_food = std::mem::replace(&mut self.food, next_food);
        return Some(ret_food)
    }
}
