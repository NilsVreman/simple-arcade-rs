// This file should contain the plugin to be used alongside bevy's AppBuilder, governing the
// Minesweeper game.

use bevy::prelude::{
    Plugin,
    App,
    OnEnter,
    OnExit,
    OnUpdate,
    IntoSystemConfigs,
    apply_system_buffers,
    IntoSystemAppConfigs
};

use arcade_util::{
    ArcadeState,
    despawn_component,
    Coord2D
};

use crate::board::{
    spawn_board,
    MinesweeperBoard,
};
use crate::systems::{
    minesweeper_game_over,
    reveal_coord,
};
use crate::minefield::{
    Minefield,
    spawn_minefield, reset_minefield,
};

pub struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Minefield::default())
            .add_systems(
                (
                    spawn_board,
                    apply_system_buffers,
                    spawn_minefield,
                ).chain()
                .in_schedule(OnEnter(ArcadeState::PlayingMinesweeper))
            )
            .add_systems(
                (
                    reveal_coord,
                    minesweeper_game_over,
                ).chain()
                .in_set(OnUpdate(ArcadeState::PlayingMinesweeper)))
            .add_systems(
                (
                    despawn_component::<Coord2D<i32>>,
                    despawn_component::<MinesweeperBoard>,
                    reset_minefield,
                ).chain()
                .in_schedule(OnExit(ArcadeState::PlayingMinesweeper)));
    }
}
