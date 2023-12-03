// This file contains the Pacman Plugin used to construct the bevy game

use arcade_util::ArcadeState;
use bevy::prelude::{
    Plugin, App, IntoSystemConfigs, OnEnter, IntoSystemAppConfigs, OnUpdate,
};

use crate::{
    board::spawn_board,
    maze::Maze,
    pacman::{Pacman, PacmanTimer, spawn_pacman, rotate_pacman, move_pacman_forward, pacman_eating}
};

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Maze::default())
            .insert_resource(Pacman::default())
            .insert_resource(PacmanTimer::default())
            .add_systems( // Things to run on starting this game
                (
                    spawn_board,
                    spawn_pacman,
                ).chain()
                .in_schedule(OnEnter(ArcadeState::PlayingPacman))
            )
            .add_systems( // Things to run whilst the game is active
                (
                    rotate_pacman,
                    move_pacman_forward,
                    pacman_eating,
                ).chain()
                .in_set(OnUpdate(ArcadeState::PlayingPacman))
            );
    }
}

