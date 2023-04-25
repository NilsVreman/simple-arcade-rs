use bevy::prelude::App;

use snake_rs::SnakePlugin;
use arcade_menu::MenuPlugin;
use arcade_util::DefaultArcadePlugin;


fn main() {
    App::new()
        .add_plugin(DefaultArcadePlugin)
        // Has to add MenuPlugin before SnakePlugin because the states are defined in MenuPlugin
        .add_plugin(MenuPlugin)
        .add_plugin(SnakePlugin)
        .run();
}
