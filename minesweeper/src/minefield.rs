use std::collections::{HashMap, HashSet};

use bevy::prelude::{
    Resource,
    Query,
    Res,
    AssetServer,
    Commands,
    BuildChildren, ResMut
};
use rand::seq::IteratorRandom;

use arcade_util::{Coord2D, CoordConfiguration};

use crate::util::{
    MineNeighbor,
    Mine,
    TILE_COLOR,
    BOARD_SIZE,
    NUM_MINES,
    Cover, Tile, MinesweeperStatus
};
use crate::board::MinesweeperBoard;

// This file should contain the Minefield struct and its associated impl block. Preferably also the
// components, resources, and systems that operate on it.

// Add revealed and flagged components
#[derive(Resource)]
pub struct Minefield {
    width: i32,
    height: i32,
    tiles: HashMap<Coord2D<i32>, Tile>,
    revealed: HashSet<Coord2D<i32>>,
    flagged: HashSet<Coord2D<i32>>,
}

// Implementation of minefield
impl Minefield {

    pub fn build(width: i32, height: i32, num_mines: i32) -> Self {
        let mut rng = rand::thread_rng();
        let tiles = HashMap::from_iter(
            (0..width).flat_map(|x| (0..height)
                    .map(move |y| (Coord2D(x, y), Tile::Empty)))
        );

        let mut minefield = Minefield {
            width,
            height,
            tiles,
            revealed: HashSet::with_capacity((width * height) as usize),
            flagged: HashSet::new(),
        };

        for coord in (0..width).flat_map(|x| (0..height).map(move |y| Coord2D(x, y)))
            .choose_multiple(&mut rng, num_mines as usize)
        {
            minefield.set_bomb(&coord);
        }

        minefield
    }

    // set a bomb at the given coordinate and update the numbers around it
    fn set_bomb(&mut self, coord: &Coord2D<i32>) {
        self.tiles.insert(*coord, Tile::Mine);
        self.neighbors(&coord)
            .iter()
            .for_each(|neighbor| {
                match self.tiles.get_mut(neighbor) {
                    Some(Tile::Number(n)) => { *n += 1; },
                    Some(Tile::Empty) => { self.tiles.insert(*neighbor, Tile::Number(1)); },
                    _ => (),
                };
            });
    }

    // Return a coordinate iterator for the neighbors to a given coordinate
    fn neighbors(&self, coord: &Coord2D<i32>) -> Vec<Coord2D<i32>> {
        let Coord2D(x, y) = coord;
        (x-1..=x+1)
            .flat_map(move |x| (y-1..=y+1)
                      .map(move |y| Coord2D(x, y)))
            .filter(move |coord| *coord != Coord2D(*x, *y) && self.is_inside(*coord))
            .collect()
    }

    // Return whether the given coordinate is inside the minefield or not
    fn is_inside(&self, coord: Coord2D<i32>) -> bool {
        coord.0 >= 0 && coord.0 < self.width && coord.1 >= 0 && coord.1 < self.height
    }

    // Reveal a tile at the given coordinate, add it to revealed, and reveal neighbors if it is
    // possible
    // Returns a set of coordinates that were revealed
    pub fn reveal_coord(&mut self, coord: &Coord2D<i32>) -> HashSet<Coord2D<i32>> {
        if self.revealed.contains(coord) {
            return HashSet::new();
        }

        let old_revealed = self.revealed.clone();

        self.revealed.insert(*coord);

        match self.tiles.get(coord) {
            Some(Tile::Empty) => {
                self.neighbors(coord)
                    .iter()
                    .for_each(|neighbor| { self.reveal_coord(neighbor); });
            },
            Some(_) => (),
            None => panic!("Tried to reveal a tile that doesn't exist"),
        }

        &self.revealed - &old_revealed
    }

    // Toggle the flag at the given coordinate, return whether the flag is now set or not
    pub fn toggle_flag(&mut self, coord: &Coord2D<i32>) -> Cover {
        if self.flagged.contains(coord) {
            self.flagged.remove(coord);
            Cover::Unflagged
        } else {
            self.flagged.insert(*coord);
            Cover::Flagged
        }
    }

    // A function which returns whether the game is won or not, alongside the outcome of the game
    // as a MinesweeperStatus enum
    pub fn game_over(&self) -> MinesweeperStatus {
        // Return MinesweeperStatus::MineTriggered if a mine is in the set of revealed tiles
        if self.revealed.iter().any(|coord| self.tiles.get(coord) == Some(&Tile::Mine)) {
            MinesweeperStatus::MineTriggered
        // Return Game won if the number of revealed tiles and flagged tiles is equal to the total
        // board size minus the number of mines
        } else if self.revealed.len()
            == (self.width * self.height) as usize
                - self.tiles.values().filter(|&tile| *tile == Tile::Mine).count() {
            MinesweeperStatus::GameWon
        } else {
            MinesweeperStatus::InProgress
        }
    }

}

impl Default for Minefield {
    fn default() -> Self {
        Minefield::build(BOARD_SIZE, BOARD_SIZE, NUM_MINES)
    }
}

impl<'a> CoordConfiguration<'a, i32> for Minefield {
    fn configuration(&'a self) -> Box<dyn Iterator<Item = &'a Coord2D<i32>> + 'a> {
        Box::new(self.tiles.keys())
    }
}

// Reset the minefield, clearing all revealed and flagged tiles and setting new mines
pub fn reset_minefield(
    mut minefield: ResMut<Minefield>,
) {
    let num_mines = minefield.tiles.values().filter(|&tile| *tile == Tile::Mine).count();

    let tiles: HashMap<Coord2D<i32>, Tile> = HashMap::from_iter(
        (0..minefield.width).flat_map(|x| (0..minefield.height)
                .map(move |y| (Coord2D(x, y), Tile::Empty)))
    );

    *minefield = Minefield {
        tiles,
        revealed: HashSet::with_capacity((minefield.width * minefield.height) as usize),
        flagged: HashSet::new(),
        ..*minefield
    };

    let mut rng = rand::thread_rng();
    for coord in (0..minefield.width).flat_map(|x| (0..minefield.height).map(move |y| Coord2D(x, y)))
        .choose_multiple(&mut rng, num_mines as usize)
    {
        minefield.set_bomb(&coord);
    }
}

// spawn the minefield components
pub fn spawn_minefield(
    mut commands: Commands,
    minefield: Res<Minefield>,
    board: Query<&MinesweeperBoard>,
    asset_server: Res<AssetServer>,
) {
    // If we currently have a board Component in the world, spawn the minefield
    if let Ok(board) = board.get_single() {

        let font = asset_server.load("fonts/pixeled.ttf");
        let img  = asset_server.load("sprites/mine.png");

        // Spawn the tiles, i.e., empty, number, or mine
        for coord in minefield.configuration() {
            let mut cmd = commands.spawn_empty();

            let tile_id = cmd.insert(
                board.tile_sprite_at_coord(coord.0, coord.1, 1, TILE_COLOR)
            )
            .insert(*coord)
            .id();

            match minefield.tiles.get(coord) {
                Some(Tile::Number(n)) => {
                    commands.entity(tile_id).insert(MineNeighbor(*n))
                        .with_children(|builder| {
                            builder.spawn(MineNeighbor(*n).to_sprite(font.clone()));
                        });
                },
                Some(Tile::Mine) => {
                    commands.entity(tile_id).insert(Mine)
                        .with_children(|builder| {
                            builder.spawn(Mine.to_sprite(img.clone()));
                        });
                },
                _ => (),
            }
        }
    }
}
