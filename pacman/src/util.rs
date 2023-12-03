// Contains constants and similar that is common among different modules

use std::time::Duration;

pub const TILE_SIZE: f32 = 30.0;
pub const TILE_SPACING: f32 = 1.0;

pub const BOARD_SIZE: i32 = 31;

pub const TICK_DURATION_MS: Duration = Duration::from_millis(150);

#[derive(Debug)]
pub enum TileType {
    Wall,
    GhostWall,
    GhostSpawn,
    Path,
    Food,
}

impl TileType {
    // A function to return the TileType from a char
    pub fn from_char(c: &char) -> Self {
        match c {
            '#' => Self::Wall,
            '?' => Self::GhostWall,
            'X' => Self::GhostSpawn,
            '.' => Self::Food,
            ' ' => Self::Path,
            _ => panic!("Invalid char: {}", c),
        }
    }
}
