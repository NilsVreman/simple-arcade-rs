fn main() {
    bevy::prelude::App::new()
        .add_plugin(arcade_util::DefaultArcadePlugin)
        .add_plugin(arcade_menu::MenuPlugin)
        .run();
}

