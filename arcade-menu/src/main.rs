use arcade_menu::MenuPlugin;
use arcade_util::{DefaultArcadePlugin, ArcadeState};
use bevy::prelude::App;

fn main() {
    App::new()
        .add_plugin(DefaultArcadePlugin)
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_state::<ArcadeState>()
        // Adds the plugins for each state
        .add_plugin(MenuPlugin)
        .run();
}

