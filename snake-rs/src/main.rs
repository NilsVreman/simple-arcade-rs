use bevy::prelude::App;

use snake_rs::SnakePlugin;
use arcade_menu::MenuPlugin;
use arcade_util::DefaultArcadePlugin;


fn main() {
    App::new()
        .add_plugin(DefaultArcadePlugin)
        .add_plugin(SnakePlugin)
        .add_plugin(MenuPlugin)
        .run();
}
