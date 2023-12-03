// Wrapper for DiscreteBoard. Keeps track of where walls, paths, similar stuff are

use std::ops::Deref;

use arcade_util::{DiscreteBoard};
use bevy::prelude::{Commands, Color, Vec2, default, BuildChildren, Component, Res};
use bevy::sprite::{SpriteBundle, Sprite};

use crate::maze::Maze;
use crate::util::{TILE_SIZE, TILE_SPACING, TileType};

const GHOST_SPAWN_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const GHOST_WALL_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);
const WALL_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);
const PATH_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const FOOD_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

const BOARD_COLOR: Color = Color::rgb(0.86, 0.77, 0.6);

#[derive(Component)]
pub struct PacmanBoard(DiscreteBoard);

impl Deref for PacmanBoard {
    type Target = DiscreteBoard;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn spawn_board(mut commands: Commands, maze: Res<Maze>) {
    let width = maze.get_width();
    let height = maze.get_height();
    let size = i32::max(width, height);

    let board = PacmanBoard(DiscreteBoard::new(size, TILE_SIZE, TILE_SPACING));
    let board_copy = PacmanBoard(DiscreteBoard::new(size, TILE_SIZE, TILE_SPACING));
    let ps = board.get_physical_size();

    commands.spawn(board)
        .insert(SpriteBundle {
            sprite: Sprite {
                color: BOARD_COLOR,
                custom_size: Some(Vec2::splat(ps)),
                ..default()
            },
            ..default()
        })
    .with_children(move |builder| {
        for y in 0..height {
            for x in 0..width {
                builder.spawn(arcade_util::Coord2D(x, y))
                    .insert(
                        board_copy.tile_sprite_at_coord(x, y, 1, match maze.get_tile_at(x, y) {
                            Some(TileType::Wall) => WALL_COLOR,
                            Some(TileType::Path) => PATH_COLOR,
                            Some(TileType::Food) => FOOD_COLOR,
                            Some(TileType::GhostSpawn) => GHOST_SPAWN_COLOR,
                            Some(TileType::GhostWall) => GHOST_WALL_COLOR,
                            None => {
                                println!("Warning: tile at ({}, {}) is None", x, y);
                                unreachable!()
                            }
                        }))
                .insert(crate::food::Food);
            }
        }
    });
}
