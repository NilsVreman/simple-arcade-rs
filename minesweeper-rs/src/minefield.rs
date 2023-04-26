use std::collections::HashSet;

use arcade_util::Coord2D;
use bevy::prelude::Resource;
use rand::seq::IteratorRandom;

// This file should contain the Minefield struct and its associated impl block. Preferably also the
// components, resources, and systems that operate on it.

// Add revealed and flagged components
#[derive(Resource)]
pub struct Minefield {
    width: i32,
    height: i32,
    mines: HashSet<Coord2D<i32>>,
    revealed: HashSet<Coord2D<i32>>,
    flagged: HashSet<Coord2D<i32>>,
}

// Implementation of minefield
impl Minefield {

    pub fn new(width: i32, height: i32, num_mines: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mines = HashSet::from_iter(
            (0..width)
                .flat_map(|x| (0..height)
                      .map(move |y| Coord2D(x, y)))
            .choose_multiple(&mut rng, num_mines as usize)
        );
        Minefield {
            width,
            height,
            mines,
            revealed: HashSet::new(),
            flagged: HashSet::new(),
        }
    }

    // Reveal the given coordinate, returning true if the player hit a mine
    pub fn reveal(&mut self, coord: &Coord2D<i32>) -> bool {
        if self.mines.contains(&coord) { return true }

        self.revealed.insert(*coord);
        if self.adjacent_mines(&coord) == 0 {
            self.neighbors(&coord)
                .iter()
                .for_each(|neighbor| {
                    self.reveal(neighbor);
                });
        }
        false
    }

    // Return a coordinate iterator for the neighbors to a given coordinate
    fn neighbors(&self, coord: &Coord2D<i32>) -> HashSet<Coord2D<i32>> {
        let Coord2D(x, y) = coord;
        (x-1..=x+1)
            .flat_map(move |x| (y-1..=y+1)
                      .map(move |y| Coord2D(x, y)))
            .filter(move |coord| *coord != Coord2D(*x, *y) && self.is_inside(*coord))
            .collect()
    }

    // Return the number of mines adjacent to the given coordinate
    fn adjacent_mines(&self, coord: &Coord2D<i32>) -> i32 {
        self.neighbors(&coord)
            .iter()
            .filter(|coord| self.mines.contains(coord))
            .count() as i32
    }

    // Return whether the given coordinate is inside the minefield or not
    fn is_inside(&self, coord: Coord2D<i32>) -> bool {
        coord.0 >= 0 && coord.0 < self.width && coord.1 >= 0 && coord.1 < self.height
    }
}
