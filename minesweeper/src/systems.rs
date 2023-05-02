use bevy::prelude::{
    Commands,
    Query,
    ResMut,
    Entity,
    EventReader,
    Res,
    AssetServer,
    NextState,
    MouseButton,
    DespawnRecursiveExt,
    BuildChildren
};
use bevy::window::Window;
use bevy::input::{ButtonState, mouse::MouseButtonInput};

use arcade_util::{Coord2D, ArcadeState};

use crate::minefield::Minefield;
use crate::util::{Cover, MinesweeperStatus, TILE_COLOR_COVERED};
use crate::board::MinesweeperBoard;

pub fn reveal_coord(
    mut commands: Commands,
    windows: Query<&Window>,
    mut minefield: ResMut<Minefield>,
    board: Query<&MinesweeperBoard>,
    covered_tiles: Query<(Entity, &Cover, &Coord2D<i32>)>,
    mut mouse_event: EventReader<MouseButtonInput>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.single();
    let board = board.single();

    for event in mouse_event.iter() {
        if let Some(pos) = window.cursor_position() {
            if let Some(coord) = board.mouse_to_coord(window, pos) {
                if board.is_coord_valid(coord.0, coord.1) {
                    handle_mouse_event(
                        &mut commands,
                        &mut minefield,
                        board,
                        &covered_tiles,
                        &coord,
                        event,
                        &asset_server,
                    );
                }
            }
        }
    }
}

// A function that uses the Minefields function "game_over" to set the next ArcadeState to
// ArcadeState::Menu
pub fn minesweeper_game_over(
    minefield: Res<Minefield>,
    mut next_state: ResMut<NextState<ArcadeState>>,
) {
    match minefield.game_over() {
        MinesweeperStatus::MineTriggered | MinesweeperStatus::GameWon => {
            next_state.set(ArcadeState::Menu);
        },
        MinesweeperStatus::InProgress => (),
    }
}

// this function handles the mouse event after being called by reveal_coord.
// If the left mousebutton is pressed, it should despawn the corresponding tile.
// If the right mousebutton is pressed, it should flag the corresponding tile by despawning the
// tile and spawning a flag sprite in its place.
fn handle_mouse_event(
    mut commands: &mut Commands,
    mut minefield: &mut ResMut<Minefield>,
    board: &MinesweeperBoard,
    covered_tiles: &Query<(Entity, &Cover, &Coord2D<i32>)>,
    pressed_coord: &Coord2D<i32>,
    mouse_event: &MouseButtonInput,
    asset_server: &Res<AssetServer>,
) {
    for (entity, _, coord) in covered_tiles.iter() {
        if coord == pressed_coord {
            match (mouse_event.state, mouse_event.button) {
                (ButtonState::Pressed, MouseButton::Left) => {
                    reveal_and_despawn_from_coord(
                        &mut commands,
                        &mut minefield,
                        &covered_tiles,
                        &pressed_coord,
                    );
                    break;
                },
                (ButtonState::Pressed, MouseButton::Right) => {
                    flag_coord(
                        &mut commands,
                        &mut minefield,
                        board,
                        &entity,
                        &pressed_coord,
                        &asset_server,
                    );
                    break;
                },
                _ => (),
            }
        }
    }
}

fn reveal_and_despawn_from_coord(
    commands: &mut Commands,
    minefield: &mut ResMut<Minefield>,
    covered_tiles: &Query<(Entity, &Cover, &Coord2D<i32>)>,
    pressed_coord: &Coord2D<i32>,
) {
    // Reveal the tile and if it is empty, reveal all adjacent tiles
    let revealed_coords = minefield.reveal_coord(pressed_coord);
    for (entity, _, coord) in covered_tiles.iter() {
        if revealed_coords.contains(coord) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn flag_coord(
    commands: &mut Commands,
    minefield: &mut ResMut<Minefield>,
    board: &MinesweeperBoard,
    entity: &Entity,
    pressed_coord: &Coord2D<i32>,
    asset_server: &Res<AssetServer>,
) {
    // load flag sprite
    let flag = asset_server.load("sprites/flag.png");
    
    // toggle flag at the given coord and if it is now flagged, spawn a flag sprite
    let cover = minefield.toggle_flag(pressed_coord);
    commands.entity(*entity).despawn_recursive();
    commands.spawn(cover.clone())
        .insert(
            board.tile_sprite_at_coord(
                pressed_coord.0,
                pressed_coord.1,
                3,
                TILE_COLOR_COVERED)
        )
        .insert(*pressed_coord)
        .with_children(|builder| {
            builder.spawn(cover.to_sprite(flag.clone()));
        });
}
