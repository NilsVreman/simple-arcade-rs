use std::ops::Deref;

use arcade_util::{DiscreteBoard, Coord2D};
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::{Component, Commands, Color, Vec2, default, BuildChildren, Transform, Query, EventReader, MouseButton, Entity, ResMut, Res, AssetServer, Handle, Image};
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::window::Window;

use crate::minefield::Minefield;
use crate::util::{BOARD_SIZE, TILE_SIZE, TILE_SPACING, TILE_COLOR_COVERED, Cover};

// this file should contain the MinesweeperBoard struct and its associated impl block. Also
// constructing the board component through the spawn_board system.

const BOARD_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

#[derive(Component)]
pub struct MinesweeperBoard(DiscreteBoard);

impl Deref for MinesweeperBoard {
    type Target = DiscreteBoard;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MinesweeperBoard {
    fn inside_board(&self, position: Vec2) -> bool {
        let board_size = self.get_physical_size();
        position.x >= -board_size / 2. && position.x <= board_size / 2. &&
            position.y >= -board_size / 2. && position.y <= board_size / 2.
    }

    pub fn mouse_to_coord(&self, window: &Window, mouse_position: Vec2) -> Option<Coord2D<i32>> {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let mouse_position = mouse_position - window_size / 2.;

        // Bounds check
        if !self.inside_board(mouse_position) {
            return None;
        }

        // World space to board space
        Some(
            Coord2D(
                self.physical_pos_to_cell_pos(mouse_position.x),
                self.physical_pos_to_cell_pos(mouse_position.y)
            )
        )
    }
}

// This function spawns the board component and returns the entity id
pub fn spawn_board(mut commands: Commands) {
    let board = MinesweeperBoard(DiscreteBoard::new(BOARD_SIZE, TILE_SIZE, TILE_SPACING));
    let board_copy = MinesweeperBoard(DiscreteBoard::new(BOARD_SIZE, TILE_SIZE, TILE_SPACING));
    let ps = board.get_physical_size();

    commands.spawn(board)
        .insert(SpriteBundle {
            sprite: Sprite {
                color: BOARD_COLOR,
                custom_size: Some(Vec2::splat(ps)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .with_children(|parent| {
            for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE {
                    parent.spawn(Cover(false))
                        .insert(
                            board_copy.tile_sprite_at_coord(x, y, 3, TILE_COLOR_COVERED)
                        )
                        .insert(Coord2D(x, y));
                }
            }
        });
    println!("Board spawned");
}

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
                        &event.state,
                        &event.button,
                        &asset_server,
                    );
                }
            }
        }
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
    mouse_event_state: &ButtonState,
    mouse_event_button: &MouseButton,
    asset_server: &Res<AssetServer>,
) {
    for (entity, cvr, coord) in covered_tiles.iter() {
        if coord == pressed_coord {
            match (mouse_event_state, mouse_event_button) {
                (ButtonState::Pressed, MouseButton::Left) => {
                    // Reveal the tile and if it is empty, reveal all adjacent tiles
                    println!("Revealing {:?}", pressed_coord);
                    reveal_and_despawn_from_coord(
                        &mut commands,
                        &mut minefield,
                        &covered_tiles,
                        &pressed_coord,
                    );
                    return;
                },
                (ButtonState::Pressed, MouseButton::Right) => {
                    println!("Flagging {:?}", pressed_coord);
                    flag_coord(
                        &mut commands,
                        &mut minefield,
                        board,
                        &covered_tiles,
                        &pressed_coord,
                        &asset_server,
                    );
                    return;
                },
                _ => (),
            }
        }
    }
}

fn reveal_and_despawn_from_coord(
    mut commands: &mut Commands,
    mut minefield: &mut ResMut<Minefield>,
    covered_tiles: &Query<(Entity, &Cover, &Coord2D<i32>)>,
    pressed_coord: &Coord2D<i32>,
) {
    // Reveal the tile and if it is empty, reveal all adjacent tiles
    let revealed_coords = minefield.reveal_coord(pressed_coord);
    for (entity, _, coord) in covered_tiles.iter() {
        if revealed_coords.contains(coord) {
            commands.entity(entity).despawn();
        }
    }
}

fn load_flag(asset_server: &Res<AssetServer>) -> Handle<Image> {
    asset_server.load("sprites/flag.png")
}

fn flag_coord(
    mut commands: &mut Commands,
    mut minefield: &mut ResMut<Minefield>,
    board: &MinesweeperBoard,
    covered_tiles: &Query<(Entity, &Cover, &Coord2D<i32>)>,
    pressed_coord: &Coord2D<i32>,
    asset_server: &Res<AssetServer>,
) {
    // load flag sprite
    let flag = load_flag(&asset_server);
    let flagged = minefield.toggle_flag(pressed_coord);

    // toggle flag at the given coord and if it is now flagged, spawn a flag sprite
    for (entity, _, coord) in covered_tiles.iter() {
        if coord == pressed_coord {
            commands.entity(entity).despawn();
            commands.spawn(Cover(flagged))
                .insert(
                    board.tile_sprite_at_coord(
                        pressed_coord.0,
                        pressed_coord.1,
                        3,
                        TILE_COLOR_COVERED)
                )
                .insert(*pressed_coord)
                .with_children(|builder| {
                    builder.spawn(Cover(flagged).to_sprite(flag.clone()));
                });
            return
        }
    }
}
