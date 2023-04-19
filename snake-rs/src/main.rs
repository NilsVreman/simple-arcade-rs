use bevy::prelude::App;

use snake_rs::SnakePlugin;

fn main() {
    App::new()
        .add_plugin(SnakePlugin)
        .run();
}
