use arcade_util::{Coord2D, CoordConfiguration};
use bevy::prelude::{Component, World, Color, Commands, Plugin, App, EventReader, Query, Res};
use bevy::ecs::system::Command;
use rand::seq::SliceRandom;

use crate::board::SnakeBoard;
use crate::snake::Snake;

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

        world.spawn(
            board.tile_sprite_at_coord(self.0.0, self.0.1, 1, FOOD_COLOR)
            )
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
    commands.add(SpawnFood(Coord2D(10, 10)));
}

pub fn food_event_listener(
    mut commands: Commands,
    query: Query<&SnakeBoard>,
    mut events: EventReader<NewFoodEvent>,
    snake: Res<Snake>,
) {
    if let Ok(board) = query.get_single() {  // To avoid panicing when we add the FoodPlugin
        let feasible_food_coord: Vec<Coord2D<i32>> = (0..board.get_size())
            .flat_map(|x| (0..board.get_size())
                 .map(move |y| Coord2D(x, y)))
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
}
