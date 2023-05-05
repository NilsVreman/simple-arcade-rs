use bevy::prelude::{
    Plugin,
    OnUpdate,
    IntoSystemConfigs,
    OnEnter,
    IntoSystemAppConfigs,
    OnExit
};

use arcade_util::{
    ArcadeState,
    despawn_component,
    Coord2D
};

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
    snake_game_over,
    reset_snake
};
use crate::board::{
    spawn_board,
    SnakeBoard,
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugin(FoodPlugin)
            .insert_resource(Snake::default())
            .insert_resource(SnakeTimer::default())
            .add_systems(  // Things to run on starting this game
                (
                    spawn_board,
                    spawn_snake,
                    spawn_food,
                )
                .chain()
                .in_schedule(OnEnter(ArcadeState::PlayingSnake)))
            .add_systems(  // Things to run whilst the game is active
                (
                    rotate_snake,
                    move_snake_forward,
                    snake_eating,
                    snake_game_over,
                )
                .chain()
                .in_set(OnUpdate(ArcadeState::PlayingSnake)))
            .add_systems(  // Things to run on exiting the game
                (
                    despawn_component::<SnakeBoard>,
                    despawn_component::<Coord2D<i32>>,
                    reset_snake,
                )
                .chain()
                .in_schedule(OnExit(ArcadeState::PlayingSnake)));
    }
}
