use bevy::prelude::{
    App,
    ResMut,
    State
};

use pacman::PacmanPlugin;
use arcade_util::{
    DefaultArcadePlugin,
    ArcadeState
};
use arcade_popup::PopupPlugin;


fn main() {
    App::new()
        .add_plugin(DefaultArcadePlugin)
        .add_plugin(PacmanPlugin)
        .add_plugin(PopupPlugin)
        .add_startup_system(set_playing_state)
        .run();
}

// Function to set the ArcadeState to PlayingSnake
pub fn set_playing_state(mut state: ResMut<State<ArcadeState>>) {
    state.0 = ArcadeState::PlayingPacman;
}
