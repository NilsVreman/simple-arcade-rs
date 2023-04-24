use arcade_menu::menu;
use arcade_util::GameState;
use bevy::{prelude::{App, Camera2dBundle, Commands, IntoSystemConfig, OnEnter, OnUpdate}, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_state::<GameState>()
        // Adds the plugins for each state
        .add_plugin(menu::MenuPlugin)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
