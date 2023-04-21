use arcade_util::Coord2D;
use bevy::prelude::{Component, World, Color, default, Transform, Vec2, Commands};
use bevy::ecs::system::Command;
use bevy::sprite::{SpriteBundle, Sprite};

use crate::board::Board;
use crate::util::TILE_SIZE;

const FOOD_COLOR: Color = Color::RED;

#[derive(Component)]
pub struct Food;

pub struct SpawnFood(pub Coord2D<i32>);

impl Command for SpawnFood {
    fn write(self, world: &mut World) {
        let board = world.query::<&Board>()
            .iter(&world)
            .next()
            .unwrap();
        let x = board.cell_pos_to_physical_pos(self.0.0);
        let y = board.cell_pos_to_physical_pos(self.0.1);

        world.spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 1.0),
            ..default()
        })
        .insert(self.0)
        .insert(Food);
    }
}

pub fn spawn_food(
    mut commands: Commands,
) {
    //let thread_rng = rand::thread_rng();
    commands.add(SpawnFood(Coord2D(15, 15)));
}
