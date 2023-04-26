use std::ops::Deref;

use arcade_util::DiscreteBoard;
use bevy::prelude::{Component, Commands, Color, Vec2, default, BuildChildren};
use bevy::sprite::{SpriteBundle, Sprite};

use crate::util::{BOARD_SIZE, TILE_SIZE, TILE_SPACING, TILE_COLOR};

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

// This function spawns the board component and returns the entity id
pub fn spawn_board(mut commands: Commands) {
    let board = MinesweeperBoard(arcade_util::DiscreteBoard::new(BOARD_SIZE, TILE_SIZE, TILE_SPACING));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: BOARD_COLOR,
            custom_size: Some(Vec2::new(
                    board.get_physical_size(),
                    board.get_physical_size())),
            ..default()
        },
        ..default()
    })
    .with_children(|builder| {
        for y in 0..board.get_size() {
            for x in 0..board.get_size() {
                builder.spawn(
                    board.tile_sprite_at_coord(x, y, TILE_COLOR)
                );
            }
        }
    })
    .insert(board);
}
