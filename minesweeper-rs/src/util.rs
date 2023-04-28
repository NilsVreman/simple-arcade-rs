use bevy::prelude::{Color, Component, Handle, default, Image, Transform, Vec2};
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::text::{TextStyle, Font, Text2dBundle, Text};

pub const TILE_SIZE: f32 = 30.0;
pub const TILE_SPACING: f32 = 1.0;
pub const TILE_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);
pub const TILE_COLOR_COVERED: Color = Color::rgb(0.4, 0.4, 0.4);

pub const FONT_SIZE: f32 = 40.0;

pub const BOARD_SIZE: i32 = 20;
pub const NUM_MINES: i32 = 60;

// Enum to describe different tile types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Mine,
    Number(i32),
    Empty,
}

// An event that is fired when a specific tile is clicked
pub struct TileClickedEvent(pub Tile);

// The component identifying a Mine tile
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
            texture: mine_image,
            ..default()
        }
    }
}

// The component identifying a Mine's Neighbor
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

// The component identifying a Covered tile
#[derive(Component, Clone)]
pub enum Cover {
    Flagged,
    Unflagged,
}

impl Cover {
    pub fn to_sprite(&self, flag_image: Handle<Image>) -> SpriteBundle {
        match self {
            Cover::Flagged => self.flagged_sprite(flag_image),
            Cover::Unflagged => self.unflagged_sprite(),
        }
    }

    fn flagged_sprite(&self, flag_image: Handle<Image>) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            texture: flag_image,
            ..default()
        }
    }

    fn unflagged_sprite(&self) -> SpriteBundle {
        SpriteBundle::default()
    }
}
