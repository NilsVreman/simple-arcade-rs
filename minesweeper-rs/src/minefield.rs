use std::collections::HashMap;

use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::{
    Resource,
    ResMut,
    Query, MouseButton, Res, Input, AssetServer, Image, Handle, Commands, BuildChildren, EventReader
};
use bevy::text::Font;
use bevy::window::Window;
use rand::seq::IteratorRandom;

use arcade_util::{Coord2D, CoordConfiguration};

use crate::util::{MineNeighbor, Mine, TILE_COLOR, BOARD_SIZE, NUM_MINES};
use crate::board::MinesweeperBoard;

// This file should contain the Minefield struct and its associated impl block. Preferably also the
// components, resources, and systems that operate on it.

// Enum to describe different tile types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Mine,
    Number(i32),
    Empty,
}

// Add revealed and flagged components
#[derive(Resource)]
pub struct Minefield {
    width: i32,
    height: i32,
    tiles: HashMap<Coord2D<i32>, Tile>,
}

// Implementation of minefield
impl Minefield {

    pub fn build(width: i32, height: i32, num_mines: i32) -> Self {
        let mut rng = rand::thread_rng();
        let tiles = HashMap::from_iter(
            (0..width).flat_map(|x| (0..height)
                    .map(move |y| (Coord2D(x, y), Tile::Empty)))
        );

        let mut minefield = Minefield { width, height, tiles };

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

    // Return the number of mines adjacent to the given coordinate
    fn adjacent_mines(&self, coord: &Coord2D<i32>) -> i32 {
        match self.tiles.get(coord) {
            Some(Tile::Number(n)) => *n,
            _ => 0,
        }
    }

    // Return whether the given coordinate is inside the minefield or not
    fn is_inside(&self, coord: Coord2D<i32>) -> bool {
        coord.0 >= 0 && coord.0 < self.width && coord.1 >= 0 && coord.1 < self.height
    }

    pub fn reset_minefield(&mut self) {
        todo!("HAVEN'T IMPLEMENTED RESET YET");
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

fn load_font(asset_server: &Res<AssetServer>) -> Handle<Font> {
    asset_server.load("fonts/pixeled.ttf")
}

fn load_mine(asset_server: &Res<AssetServer>) -> Handle<Image> {
    asset_server.load("sprites/mine.png")
}

// spawn the minefield components
pub fn spawn_minefield(
    mut commands: Commands,
    mut minefield: ResMut<Minefield>,
    board: Query<&MinesweeperBoard>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(board) = board.get_single() {

        let font = load_font(&asset_server);
        let img  = load_mine(&asset_server);

        for coord in minefield.configuration() {
            let mut cmd = commands.spawn_empty();

            let tile_id = cmd.insert(
                board.tile_sprite_at_coord(coord.0, coord.1, TILE_COLOR)
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

pub fn reveal_coord(
    windows: Query<&Window>,
    board: Query<&MinesweeperBoard>,
    mut mouse_event: EventReader<MouseButtonInput>,
) {
    let window = windows.single();
    let board = board.single();
    for event in mouse_event.iter() {
        if let Some(pos) = window.cursor_position() {
            if let Some(coord) = board.mouse_to_coord(window, pos) {
                match (event.state, event.button) {
                    (ButtonState::Pressed, MouseButton::Left) => {
                        println!("Revealing {:?}", coord);
                    },
                    (ButtonState::Pressed, MouseButton::Right) => {
                        println!("Flagging {:?}", coord);
                    },
                    _ => (),
                }
            }
        }
    }
}
