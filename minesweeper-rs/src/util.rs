use bevy::ecs::system::Command;
use bevy::prelude::{Color, Query, Component, Handle, default, Image, Transform, Vec2, World, BuildWorldChildren, Res, AssetServer};
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::text::{TextStyle, Font, Text2dBundle, Text};

use crate::board::MinesweeperBoard;

pub const TILE_SIZE: f32 = 30.0;
pub const TILE_SPACING: f32 = 1.0;
pub const TILE_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);

pub const FONT_SIZE: f32 = 40.0;

pub const BOARD_SIZE: i32 = 20;
pub const NUM_MINES: i32 = 40;

#[derive(Component)]
pub struct Mine;

impl Mine {
    pub fn to_sprite(&self, mine_image: Handle<Image>) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            texture: mine_image.clone(),
            ..default()
        }
    }
}

#[derive(Component)]
pub struct MineNeighbor(pub i32);

impl MineNeighbor {
    pub fn to_sprite(&self, font: Handle<Font>) -> Text2dBundle {
        let (text, color) = match self {
            MineNeighbor(1) => ("1", Color::rgb(0.0, 0.0, 1.0)),
            MineNeighbor(2) => ("2", Color::rgb(0.0, 0.5, 0.0)),
            MineNeighbor(3) => ("3", Color::rgb(1.0, 0.0, 0.0)),
            MineNeighbor(4) => ("4", Color::rgb(0.0, 0.0, 0.5)),
            MineNeighbor(5) => ("5", Color::rgb(0.5, 0.0, 0.0)),
            MineNeighbor(6) => ("6", Color::rgb(0.0, 0.5, 0.5)),
            MineNeighbor(7) => ("7", Color::rgb(0.5, 0.5, 0.0)),
            MineNeighbor(8) => ("8", Color::rgb(0.5, 0.0, 0.5)),
            _ => ("", Color::rgb(0.0, 0.0, 0.0)),
        };

        Text2dBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font,
                    font_size: FONT_SIZE,
                    color
                }),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        }
        
    }
}

#[derive(Component)]
pub struct Uncover;
