use bevy::prelude::App;

use snake::SnakePlugin;
use minesweeper::MinesweeperPlugin;
use arcade_menu::MenuPlugin;
use arcade_util::DefaultArcadePlugin;
use arcade_popup::PopupPlugin;


fn main() {
    App::new()
        .add_plugin(DefaultArcadePlugin)
        .add_plugin(PopupPlugin)
        // Has to add MenuPlugin before SnakePlugin because the states are defined in MenuPlugin
        .add_plugin(MenuPlugin)
        .add_plugin(SnakePlugin)
        .add_plugin(MinesweeperPlugin)
        .run();
}
