use bevy::prelude::{
    App,
    ResMut,
    State
};

use snake::SnakePlugin;
use arcade_util::{
    DefaultArcadePlugin,
    ArcadeState
};


fn main() {
    App::new()
        .add_plugin(DefaultArcadePlugin)
        .add_plugin(SnakePlugin)
        .add_startup_system(set_playing_state)
        .run();
}

// Function to set the ArcadeState to PlayingSnake
pub fn set_playing_state(mut state: ResMut<State<ArcadeState>>) {
    state.0 = ArcadeState::PlayingSnake;
}
