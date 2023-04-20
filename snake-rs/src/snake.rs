use std::collections::VecDeque;

use bevy::prelude::{Color, Resource, Commands, Res, Vec2, Transform};
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::utils::default;

use arcade_util::{Coord2D, CoordConfiguration};
use crate::board::Board;
use crate::util::{TILE_SIZE};

const SNAKE_COLOR: Color = Color::rgb(0.42, 0.63, 0.07);

#[derive(Resource)]
pub struct Snake {
    segments: VecDeque<Coord2D<u8>>,
}

impl<'a> CoordConfiguration<'a, u8> for Snake {
    fn configuration(&'a self) -> Box<dyn Iterator<Item = &'a Coord2D<u8>> + 'a> {
        Box::new(self.segments.iter())
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            segments: VecDeque::from([Coord2D(1, 1)]),
        }
    }
}

pub fn spawn_snake(mut commands: Commands, snake: Res<Snake>) {
    for segment in snake.configuration() {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                           Board::cell_pos_to_physical_pos(segment.0),
                           Board::cell_pos_to_physical_pos(segment.1),
                           1.0),
            ..default()
        })
        .insert(segment.clone());
    }
}
