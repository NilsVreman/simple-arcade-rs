use bevy::prelude::{App};
use minesweeper_rs::MinesweeperPlugin;
use arcade_util::DefaultArcadePlugin;

fn main() {
    App::new()
        .add_plugin(DefaultArcadePlugin)
        .add_plugin(MinesweeperPlugin)
        .run();
}
