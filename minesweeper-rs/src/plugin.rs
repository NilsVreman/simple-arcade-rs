// This file should contain the plugin to be used alongside bevy's AppBuilder, governing the
// Minesweeper game.

use arcade_util::ArcadeState;
use bevy::prelude::{Plugin, App, IntoSystemAppConfigs, OnEnter, OnExit, OnUpdate};

use crate::board::spawn_board;

pub struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(
        //    (
        //        spawn_board,
        //    ).in_schedule(OnEnter(ArcadeState::PlayingMinesweeper))
        //);
        app.add_system(spawn_board);
        todo!("ADD SYSTEMS AND SPRITES TO THE MINESWEEPERPLUGIN");
    }
}
