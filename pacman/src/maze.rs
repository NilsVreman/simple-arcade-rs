// implementation of the maze resource keeping track of the food, walls, and paths.

use std::collections::HashMap;

use arcade_util::Coord2D;
use bevy::prelude::Resource;

use crate::util::TileType;

const MAZE_STRING: &str = "\
####################
#......#....#......#
#.####.#.##.#.####.#
#.#....#....#....#.#
#.#.##.#.##.#.##.#.#
#.#.##...##......#.#
#....##########....#
####.#........#.#.##
#....#.??????.#.#..#
#.####.?XXXX?...##.#
#.##...?XXXX?.#.#..#
#.##.#.??????.#.#.##
#....#........#.#..#
#.#############.##.#
#........#.........#
###.####.#.#####.###
#......#.#.#.......#
#.##.#.#.#.#.#.###.#
#....#.......#.....#
####################";

#[derive(Resource)]
pub struct Maze {
    width: i32,
    height: i32,
    maze: HashMap<Coord2D<i32>, TileType>,
}

impl Default for Maze {
    fn default() -> Self {
        let width = MAZE_STRING.lines().next().unwrap().len() as i32;
        let height = MAZE_STRING.lines().count() as i32;

        let maze: HashMap<Coord2D<i32>, TileType> = MAZE_STRING
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line
                    .chars()
                    .enumerate()
                    .map(move |(x, c)| {
                        (Coord2D(x as i32, y as i32), TileType::from_char(&c))
                    })
                })
            .collect();

        Self { width, height, maze, }
    }
}

impl Maze {
    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_tile(&self, coord: Coord2D<i32>) -> Option<&TileType> {
        self.maze.get(&coord)
    }

    pub fn get_tile_at(&self, x: i32, y: i32) -> Option<&TileType> {
        self.get_tile(Coord2D(x, y))
    }

    pub fn eat_food_at(&mut self, x: i32, y: i32) -> bool {
        if let Some(tile) = self.maze.get_mut(&Coord2D(x, y)) {
            if let TileType::Food = tile {
                *tile = TileType::Path;
                return true;
            }
        };
        return false;
    }

    pub fn collides_with(&self, coord: &Coord2D<i32>) -> bool {
        return match self.maze.get(coord) {
            Some(TileType::Path) | Some(TileType::Food) => false,
            _ => true,
        }
    }
}
