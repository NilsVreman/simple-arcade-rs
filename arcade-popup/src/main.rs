use bevy::prelude::{ResMut, State, Commands};

fn main() {
    bevy::prelude::App::new()
        .add_plugin(arcade_util::DefaultArcadePlugin)
        .add_plugin(arcade_popup::PopupPlugin)
        .add_startup_system(set_popup_state)
        .add_startup_system(generate_popup)
        .run();
}

// Set state to the correct state, i.e., ArcadeState::Result
pub fn set_popup_state(mut state: ResMut<State<arcade_util::ArcadeState>>) {
    state.0 = arcade_util::ArcadeState::Popup;
}

pub fn generate_popup(
    commands: Commands,
) {
    arcade_popup::spawn_popup(
        commands,
        String::from("Test Header"),
        String::from("Test Body"),
    );
}
