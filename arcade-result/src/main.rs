use bevy::prelude::{ResMut, State};

fn main() {
    bevy::prelude::App::new()
        .add_plugin(arcade_util::DefaultArcadePlugin)
        .add_plugin(arcade_result::MessageResultPlugin)
        .add_startup_system(set_result_state)
        .run();
}

// Set state to the correct state, i.e., ArcadeState::Result
pub fn set_result_state(mut state: ResMut<State<arcade_util::ArcadeState>>) {
    state.0 = arcade_util::ArcadeState::Result;
}
