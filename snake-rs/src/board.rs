use bevy::prelude::{Color, Component, Commands, Vec2, default, BuildChildren, Transform};
use bevy::sprite::{SpriteBundle, Sprite};

use crate::util::{TILE_SIZE, BOARD_SIZE, TILE_SPACING};

const BOARD_COLOR: Color = Color::rgb(0.86, 0.77, 0.6);
const BOARD_COLOR_LIGHT: Color = Color::rgb(0.85, 0.74, 0.56);

#[derive(Component)]
pub struct Board;

impl Board {
    fn pos_to_physical(pos: u8) -> f32 {
        pos as f32 * TILE_SIZE + (pos + 1) as f32 * TILE_SPACING
    }

    pub fn cell_pos_to_physical_pos(pos: u8) -> f32 {
        let offset = -Self::pos_to_physical(BOARD_SIZE) / 2.0 + 0.5 * TILE_SIZE;
        offset + Self::pos_to_physical(pos)
    }
}

pub fn spawn_board(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: BOARD_COLOR,
            custom_size: Some(Vec2::new(
                    Board::pos_to_physical(BOARD_SIZE),
                    Board::pos_to_physical(BOARD_SIZE))),
            ..default()
        },
        ..default()
    })
    .with_children(|builder| {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
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
                                   Board::cell_pos_to_physical_pos(x),
                                   Board::cell_pos_to_physical_pos(y),
                                   1.0),
                    ..default()
                });
            }
        }
    })
    .insert(Board);
}
