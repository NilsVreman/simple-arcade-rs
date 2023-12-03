// This file is primarily used for the control of the pacman character

use bevy::ecs::system::Command;
use bevy::prelude::{Resource, Res, Input, KeyCode, ResMut, Commands, Query, Entity, World, Color, With};
use bevy::time::{Timer, TimerMode, Time};

use arcade_util::{Dir2D, Coord2D};

use crate::board::PacmanBoard;
use crate::food::Food;
use crate::maze::Maze;
use crate::util::TICK_DURATION_MS;

const PACMAN_COLOR: Color = Color::rgb(1.0, 1.0, 0.0);

#[derive(Resource)]
pub struct Pacman {
    pos: Coord2D<i32>,
    dir: Dir2D,
}

impl Default for Pacman {
    fn default() -> Self {
        Self {
            pos: Coord2D(1, 18),
            dir: Dir2D::Right,
        }
    }
}

impl Pacman {
    // This function moves the pacman one step in the direction he's facing.
    // NOTE: He does not consider whether there is a wall in front of him or not.
    pub fn step_once(&mut self) {
        self.pos = self.pos + self.dir.as_coord();
    }

    pub fn set_dir(&mut self, dir: Dir2D) {
        self.dir = dir;
    }

    pub fn get_pos(&self) -> Coord2D<i32> {
        return self.pos
    }

    fn get_next_pos(&self) -> Coord2D<i32> {
        return self.pos + self.dir.as_coord();
    }
}

// Timer //

#[derive(Resource)]
pub struct PacmanTimer(pub Timer);

impl Default for PacmanTimer {
    fn default() -> Self {
        PacmanTimer(Timer::new(TICK_DURATION_MS, TimerMode::Repeating))
    }
}

pub fn spawn_pacman(
    mut commands: Commands,
    pacman: Res<Pacman>,
) {
    commands.add(SpawnPacmanSegment(pacman.get_pos()));
}

pub fn move_pacman_forward(
    time: Res<Time>,
    mut timer: ResMut<PacmanTimer>,
    mut commands: Commands,
    mut pacman: ResMut<Pacman>,
    maze: Res<Maze>,
    coords: Query<(Entity, &Coord2D<i32>)>,
) {
    if timer.0.tick(time.delta()).finished() &&
        !maze.collides_with(&pacman.get_next_pos())
    {
        let old_pos = pacman.get_pos();
        pacman.step_once();
        commands.add(SpawnPacmanSegment(pacman.get_pos())); // Spawn sprite

        if let Some((entity, _)) = coords.iter().find(|(_, &coord)| coord == old_pos) {
            commands.entity(entity).despawn(); // Remove the visual sprite as well
        }
    }
}

pub fn rotate_pacman(
    input: Res<Input<KeyCode>>,
    mut pacman: ResMut<Pacman>,
) {
    if input.any_pressed([KeyCode::H, KeyCode::Left]) {
        pacman.set_dir(Dir2D::Left);
    } else if input.any_pressed([KeyCode::J, KeyCode::Down]) {
        pacman.set_dir(Dir2D::Down);
    } else if input.any_pressed([KeyCode::K, KeyCode::Up]) {
        pacman.set_dir(Dir2D::Up);
    } else if input.any_pressed([KeyCode::L, KeyCode::Right]) {
        pacman.set_dir(Dir2D::Right);
    }
}

pub fn pacman_eating(
    mut commands: Commands,
    pacman: ResMut<Pacman>,
    mut food: Query<(Entity, &Coord2D<i32>), With<Food>>,
) {
    let pacman_pos = pacman.get_pos();
    for food in food.iter() {
        println!("food: {:?}", food);
    }
    if let Some((entity, _)) = food.iter().find(|(_, &coord)| coord == pacman_pos) {
        println!("Pacman ate food at {:?}", pacman_pos);
        commands.entity(entity).despawn(); // Despawn apple and add last element to tail
    }
}

// We use the SpawnPacmanSegment as a messenger that we need to spawn a sprite at the given coord
pub struct SpawnPacmanSegment(pub Coord2D<i32>);

// Command is a way to mutate the World 
impl Command for SpawnPacmanSegment {
    fn write(self, world: &mut World) {
        let board = world.query::<&PacmanBoard>()
            .iter(&world)
            .next()
            .unwrap();

        world.spawn(board.tile_sprite_at_coord(self.0.0, self.0.1, 1, PACMAN_COLOR))
            .insert(self.0);
    }
}
