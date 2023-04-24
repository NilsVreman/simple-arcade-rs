use arcade_util::GameState;
use bevy::prelude::{Plugin, OnUpdate, IntoSystemConfigs};

use crate::food::{
    FoodPlugin,
    spawn_food
};
use crate::snake::{
    Snake,
    SnakeTimer,
    spawn_snake,
    rotate_snake,
    move_snake_forward,
    snake_eating,
    snake_game_over
};
use crate::board::spawn_board;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugin(FoodPlugin)
            .insert_resource(Snake::default())
            .insert_resource(SnakeTimer::default())
            .add_state::<GameState>()
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
                .chain()
                .in_set(OnUpdate(GameState::Snake)));
    }
}
