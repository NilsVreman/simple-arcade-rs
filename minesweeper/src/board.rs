use std::ops::Deref;

use arcade_util::{DiscreteBoard, Coord2D};
use bevy::prelude::{Component, Commands, Color, Vec2, default, BuildChildren, Transform};
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::window::Window;

use crate::util::{BOARD_SIZE, TILE_SIZE, TILE_SPACING, TILE_COLOR_COVERED, Cover};

// this file should contain the MinesweeperBoard struct and its associated impl block. Also
// constructing the board component through the spawn_board system.

const BOARD_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

#[derive(Component)]
pub struct MinesweeperBoard(DiscreteBoard);

impl Deref for MinesweeperBoard {
    type Target = DiscreteBoard;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MinesweeperBoard {
    fn inside_board(&self, position: Vec2) -> bool {
        let board_size = self.get_physical_size();
        position.x >= -board_size / 2. && position.x <= board_size / 2. &&
            position.y >= -board_size / 2. && position.y <= board_size / 2.
    }

    pub fn mouse_to_coord(&self, window: &Window, mouse_position: Vec2) -> Option<Coord2D<i32>> {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let mouse_position = mouse_position - window_size / 2.;

        // Bounds check
        if !self.inside_board(mouse_position) {
            return None;
        }

        // World space to board space
        Some(
            Coord2D(
                self.physical_pos_to_cell_pos(mouse_position.x),
                self.physical_pos_to_cell_pos(mouse_position.y)
            )
        )
    }
}

// This function spawns the board component and returns the entity id
pub fn spawn_board(mut commands: Commands) {
    let board = MinesweeperBoard(DiscreteBoard::new(BOARD_SIZE, TILE_SIZE, TILE_SPACING));
    let board_copy = MinesweeperBoard(DiscreteBoard::new(BOARD_SIZE, TILE_SIZE, TILE_SPACING));
    let ps = board.get_physical_size();

    commands.spawn(board)
        .insert(SpriteBundle {
            sprite: Sprite {
                color: BOARD_COLOR,
                custom_size: Some(Vec2::splat(ps)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .with_children(|parent| {
            for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE {
                    parent.spawn(Cover::Unflagged)
                        .insert(
                            board_copy.tile_sprite_at_coord(x, y, 3, TILE_COLOR_COVERED)
                        )
                        .insert(Coord2D(x, y));
                }
            }
        });
}
