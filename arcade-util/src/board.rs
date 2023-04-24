use bevy::prelude::Component;

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

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_physical_size(&self) -> f32 {
        self.physical_size
    }
}
