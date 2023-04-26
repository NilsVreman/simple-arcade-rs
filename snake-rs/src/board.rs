use std::ops::Deref;

use arcade_util::{DiscreteBoard, Collidable};
use bevy::prelude::{Color, Component, Commands, Vec2, default, BuildChildren};
use bevy::sprite::{SpriteBundle, Sprite};

use crate::util::{TILE_SIZE, TILE_SPACING, BOARD_SIZE};

const BOARD_COLOR: Color = Color::rgb(0.86, 0.77, 0.6);
const BOARD_COLOR_LIGHT: Color = Color::rgb(0.85, 0.74, 0.56);

// Wrapper for DiscreteBoard

#[derive(Component)]
pub struct SnakeBoard(DiscreteBoard);

impl Deref for SnakeBoard {
    type Target = DiscreteBoard;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Collidable<i32> for SnakeBoard {
    fn collides_with(&self, coord: &arcade_util::Coord2D<i32>) -> bool {
        coord.0 >= self.get_size()
            || coord.0 < 0
            || coord.1 >= self.get_size()
            || coord.1 < 0
    }
}

pub fn spawn_board(mut commands: Commands) {
    let board = SnakeBoard(DiscreteBoard::new(BOARD_SIZE, TILE_SIZE, TILE_SPACING));

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
                    board.tile_sprite_at_coord(x, y, if (x+y) % 2 == 0 {
                        BOARD_COLOR
                    } else {
                        BOARD_COLOR_LIGHT
                    }));
            }
        }
    })
    .insert(board);
}
