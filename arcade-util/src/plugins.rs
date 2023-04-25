use bevy::{
    prelude::{
        Plugin,
        App,
        AssetPlugin,
        default,
        PluginGroup,
        Commands,
        Camera2dBundle,
    },
    DefaultPlugins
};

use crate::ArcadeState;

pub struct DefaultArcadePlugin;

impl Plugin for DefaultArcadePlugin {
    fn build(&self, app: &mut App) {
        // Load assets from the correct folder meaning that we use the same asset library for all
        // of the arcade games
        app.add_plugins(DefaultPlugins.set(AssetPlugin {
            asset_folder: "../assets/".to_string(),
            ..default()
        }))
        .add_state::<ArcadeState>()
        // Add camera to the system
        .add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
