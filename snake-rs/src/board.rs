use arcade_util::Collidable;
use bevy::prelude::{Color, Component, Commands, Vec2, default, BuildChildren, Transform};
use bevy::sprite::{SpriteBundle, Sprite};

use crate::util::{TILE_SIZE, TILE_SPACING, BOARD_SIZE};

const BOARD_COLOR: Color = Color::rgb(0.86, 0.77, 0.6);
const BOARD_COLOR_LIGHT: Color = Color::rgb(0.85, 0.74, 0.56);

#[derive(Component)]
pub struct Board {
    size: i32,
    physical_size: f32,
}

impl Board {
    pub fn new(size: i32) -> Self {
        let physical_size = Self::pos_to_physical(size);
        Self { size, physical_size }
    }

    fn pos_to_physical(pos: i32) -> f32 {
        pos as f32 * TILE_SIZE + (pos + 1) as f32 * TILE_SPACING
    }

    pub fn cell_pos_to_physical_pos(&self, pos: i32) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * TILE_SIZE;
        offset + Self::pos_to_physical(pos)
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }
}

impl Collidable<i32> for Board {
    fn collides_with(&self, coord: &arcade_util::Coord2D<i32>) -> bool {
        return coord.0 < 0
            || coord.0 >= self.size
            || coord.1 < 0
            || coord.1 >= self.size
    }
}

pub fn spawn_board(mut commands: Commands) {
    let board = Board::new(BOARD_SIZE);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: BOARD_COLOR,
            custom_size: Some(Vec2::new(
                    board.physical_size,
                    board.physical_size)),
            ..default()
        },
        ..default()
    })
    .with_children(|builder| {
        for y in 0..board.size {
            for x in 0..board.size {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: if (x+y) % 2 == 0 {
                            BOARD_COLOR
                        } else {
                            BOARD_COLOR_LIGHT
                        },
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                                   board.cell_pos_to_physical_pos(x),
                                   board.cell_pos_to_physical_pos(y),
                                   1.0),
                    ..default()
                });
            }
        }
    })
    .insert(board);
}
