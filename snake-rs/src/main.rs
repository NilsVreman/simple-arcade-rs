use bevy::prelude::*;

use arcade_util::GameState;
use snake_rs::board::spawn_board;
use snake_rs::food::{spawn_food, FoodPlugin};
use snake_rs::snake::{
    Snake,
    spawn_snake,
    SnakeTimer,
    move_snake_forward,
    rotate_snake,
    snake_eating,
    snake_game_over,
};
use snake_rs::util::TICK_DURATION_MS;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FoodPlugin)
        .insert_resource(Snake::default())
        .insert_resource(SnakeTimer::default())
        .add_state::<GameState>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_startup_system(spawn_snake)
        .add_startup_system(spawn_food)
        .add_systems(
            (
                rotate_snake,
                move_snake_forward,
                snake_eating,
                snake_game_over,
            )
            .in_set(OnUpdate(GameState::Playing)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
