use arcade_util::{Coord2D, CoordConfiguration};
use bevy::prelude::{Component, World, Color, default, Transform, Vec2, Commands, Plugin, App, EventReader, Query, Res};
use bevy::ecs::system::Command;
use bevy::sprite::{SpriteBundle, Sprite};
use rand::seq::SliceRandom;

use crate::board::SnakeBoard;
use crate::snake::Snake;
use crate::util::TILE_SIZE;

const FOOD_COLOR: Color = Color::RED;

#[derive(Component)]
pub struct Food;

pub struct SpawnFood(pub Coord2D<i32>);

impl Command for SpawnFood {
    fn write(self, world: &mut World) {
        let board = world.query::<&SnakeBoard>()
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

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewFoodEvent>()
            .add_system(food_event_listener);
    }
}

pub struct NewFoodEvent;

pub fn spawn_food(
    mut commands: Commands,
) {
    //let thread_rng = rand::thread_rng();
    commands.add(SpawnFood(Coord2D(10, 10)));
}

pub fn food_event_listener(
    mut commands: Commands,
    query: Query<&SnakeBoard>,
    mut events: EventReader<NewFoodEvent>,
    snake: Res<Snake>,
) {
    let board = query.single();

    let feasible_food_coord: Vec<Coord2D<i32>> = (0..board.get_size())
        .into_iter()
        .map(|x| (0..board.get_size())
             .into_iter()
             .map(move |y| Coord2D(x, y)))
        .flatten()
        .filter(|c| !snake.configuration().any(|sc| sc == c))
        .collect();

    // Need to do this to consume the events in the EventReader, and also add that much new food
    let mut num_food = 0;
    for _ in events.iter() {
        num_food += 1;
    }

    let mut rng = rand::thread_rng();
    for &coord in feasible_food_coord.choose_multiple(&mut rng, num_food) {
        commands.add(SpawnFood(coord));
    }
}
