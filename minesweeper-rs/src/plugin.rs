// This file should contain the plugin to be used alongside bevy's AppBuilder, governing the
// Minesweeper game.

use arcade_util::ArcadeState;
use bevy::prelude::{Plugin, App, OnEnter, OnExit, OnUpdate, IntoSystemConfigs, apply_system_buffers};

use crate::board::{
    spawn_board,
    reveal_coord,
};
use crate::minefield::{
    Minefield,
    spawn_minefield,
};

pub struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Minefield::default())
            .add_startup_systems(
                (
                    spawn_board,
                    apply_system_buffers,
                    spawn_minefield,
                ).chain()
            )
            .add_system(reveal_coord);
        //todo!("ADD SYSTEMS AND SPRITES TO THE MINESWEEPERPLUGIN");
    }
}
