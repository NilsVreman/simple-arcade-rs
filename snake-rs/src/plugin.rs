use arcade_util::ArcadeState;
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
                .in_set(OnUpdate(ArcadeState::Playing)));
    }
    //        // Systems to handle the main menu screen
    //        .add_system(menu_setup.in_schedule(OnEnter(ArcadeState::Menu)))
    //        .add_system(main_menu_setup.in_schedule(OnEnter(MenuState::Main)))
    //        .add_system(despawn_component::<OnMainMenuScreen>.in_schedule(OnExit(MenuState::Main)))
    //        // Systems to handle the game list menu screen
    //        .add_system(game_list_setup.in_schedule(OnEnter(MenuState::GameSelection)))
    //        .add_system(despawn_component::<OnGamesMenuScreen>.in_schedule(OnExit(MenuState::GameSelection)))
    //        // Common systems to all screens that handles buttons behavior
    //        .add_systems(
    //            (
    //                menu_action,
    //                game_list_action,
    //                button_system,
    //                keybinding_system,
    //            )
    //            .in_set(OnUpdate(ArcadeState::Menu)),
}
