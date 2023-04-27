use bevy::prelude::{Component, Color, default, Transform, Vec2};
use bevy::sprite::{SpriteBundle, Sprite};

#[derive(Component)]
pub struct DiscreteBoard {
    tile_size: f32,
    tile_spacing: f32,
    size: i32,
    physical_size: f32,
}

impl DiscreteBoard {
    pub fn new(size: i32, tile_size: f32, tile_spacing: f32) -> Self {
        let physical_size = Self::pos_to_physical(size, tile_size, tile_spacing);
        Self { tile_size, tile_spacing, size, physical_size }
    }

    fn pos_to_physical(pos: i32, tile_size: f32, tile_spacing: f32) -> f32 {
        pos as f32 * tile_size + (pos + 1) as f32 * tile_spacing
    }

    pub fn cell_pos_to_physical_pos(&self, pos: i32) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * self.tile_size;
        offset + Self::pos_to_physical(pos, self.tile_size, self.tile_spacing)
    }

    pub fn physical_pos_to_cell_pos(&self, pos: f32) -> i32 {
        let offset = -self.physical_size / 2.0 + 0.5 * self.tile_size;
        ((pos - offset - self.tile_spacing) / (self.tile_size + self.tile_spacing))
            .round() as i32
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_physical_size(&self) -> f32 {
        self.physical_size
    }

    pub fn tile_sprite_at_coord(&self, x: i32, y: i32, color: Color) -> SpriteBundle {
        let x = self.cell_pos_to_physical_pos(x);
        let y = self.cell_pos_to_physical_pos(y);
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(self.tile_size, self.tile_size)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 1.0),
            ..default()
        }
    }
}
