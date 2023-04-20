use bevy::prelude::*;

use snake_rs::board::spawn_board;
use snake_rs::snake::{Snake, spawn_snake};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Snake::default())
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_startup_system(spawn_snake)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
