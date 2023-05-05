use bevy::prelude::{ResMut, State, Commands};

fn main() {
    bevy::prelude::App::new()
        .add_plugin(arcade_util::DefaultArcadePlugin)
        .add_plugin(arcade_result::MessageResultPlugin)
        .add_startup_system(set_result_state)
        .add_startup_system(send_message_result)
        .run();
}

// Set state to the correct state, i.e., ArcadeState::Result
pub fn set_result_state(mut state: ResMut<State<arcade_util::ArcadeState>>) {
    state.0 = arcade_util::ArcadeState::Result;
}

pub fn send_message_result(
    commands: Commands,
) {
    arcade_result::add_message_result(
        commands,
        String::from("Test Header"),
        String::from("Test Body"),
    );
}
