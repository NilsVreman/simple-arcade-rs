use std::collections::VecDeque;

use bevy::ecs::system::Command;
use bevy::prelude::{Color, Resource, Commands, Res, Vec2, Transform, ResMut, Query, Entity, World, Input, KeyCode, With, EventWriter, NextState};
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::time::{Time, Timer, TimerMode};
use bevy::utils::default;

use arcade_util::{Coord2D, CoordConfiguration, Dir2D, GameState, Collidable};
use crate::food::{Food, NewFoodEvent};
use crate::board::SnakeBoard;
use crate::util::{TILE_SIZE, MIN_TIMER_DURATION, TICK_DURATION_MS, GameCompletionEvent, GameOver};

const SNAKE_COLOR: Color = Color::rgb(0.42, 0.63, 0.07);

// Snake //

#[derive(Resource)]
pub struct Snake {
    segments: VecDeque<Coord2D<i32>>,
    direction: Dir2D,
    old_tail: Coord2D<i32>,
}

impl<'a> CoordConfiguration<'a, i32> for Snake {
    fn configuration(&'a self) -> Box<dyn Iterator<Item = &'a Coord2D<i32>> + 'a> {
        Box::new(self.segments.iter())
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            segments: VecDeque::from([Coord2D(1, 1)]),
            direction: Dir2D::Up,
            old_tail: Coord2D(1, 0),
        }
    }
}

impl Snake {
    pub fn grow(&mut self) {
        self.segments.push_back(self.old_tail);
    }

    pub fn step_once(&mut self) {
    // Move head in direction
        let mut next_coord = self.segments[0] + self.direction.as_coord();
        self.segments.push_front(next_coord); // Add new head to start of snake
        self.old_tail = self.segments.pop_back().unwrap(); // Remove old tail
    }

    pub fn get_head(&self) -> Coord2D<i32> {
        self.segments[0]
    }

    pub fn get_old_tail(&self) -> Coord2D<i32> {
        self.old_tail
    }
}

// We use the SpawnSnakeSegment as a messenger that we need to spawn a sprite at the given coord
pub struct SpawnSnakeSegment(pub Coord2D<i32>);

// Command is a way to mutate the World 
impl Command for SpawnSnakeSegment {
    fn write(self, world: &mut World) {
        let board = world.query::<&SnakeBoard>()
            .iter(&world)
            .next()
            .unwrap();
        let x = board.cell_pos_to_physical_pos(self.0.0);
        let y = board.cell_pos_to_physical_pos(self.0.1);

        world.spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 1.0),
            ..default()
        })
        .insert(self.0);
    }
}

// Timer //

#[derive(Resource)]
pub struct SnakeTimer(pub Timer);

impl SnakeTimer {
    fn update_timer(&mut self) {
        if self.0.duration() > MIN_TIMER_DURATION {
            self.0 = Timer::new(self.0.duration() / 2, TimerMode::Repeating);
        }
    }
}

impl Default for SnakeTimer {
    fn default() -> Self {
        SnakeTimer(Timer::new(TICK_DURATION_MS, TimerMode::Repeating))
    }
}

// Systems //

pub fn spawn_snake(
    mut commands: Commands,
    snake: Res<Snake>
) {
    for segment in snake.configuration() {
        commands.add(SpawnSnakeSegment(*segment));
    }
}

pub fn move_snake_forward(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    coords: Query<(Entity, &Coord2D<i32>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        snake.step_once();
        commands.add(SpawnSnakeSegment(snake.get_head())); // Spawn sprite

        let old_tail = snake.get_old_tail();
        if let Some((entity, _)) = coords.iter().find(|(_, &coord)| coord == old_tail) {
            commands.entity(entity).despawn(); // Remove the visual sprite as well
        }
    }
}

pub fn rotate_snake(
    input: Res<Input<KeyCode>>,
    mut snake: ResMut<Snake>,
) {
    if input.pressed(KeyCode::H) {
        if snake.direction == Dir2D::Right { return }
        snake.direction = Dir2D::Left;
    }
    if input.pressed(KeyCode::J) {
        if snake.direction == Dir2D::Up { return }
        snake.direction = Dir2D::Down;
    } 
    if input.pressed(KeyCode::K) {
        if snake.direction == Dir2D::Down { return }
        snake.direction = Dir2D::Up;
    }
    if input.pressed(KeyCode::L) {
        if snake.direction == Dir2D::Left { return }
        snake.direction = Dir2D::Right;
    }
}

pub fn snake_eating(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    food: Query<(Entity, &Coord2D<i32>), With<Food>>,
    mut event: EventWriter<NewFoodEvent>,
) {
    let snake_head = snake.get_head();
    if let Some((entity, _)) = food.iter().find(|(_, &coord)| coord == snake_head) {
        commands.entity(entity).despawn(); // Despawn apple and add last element to tail
        snake.grow();
        commands.add(SpawnSnakeSegment(snake.get_old_tail()));
        event.send(NewFoodEvent);
    }
}

pub fn snake_game_over(
    mut commands: Commands,
    snake: Res<Snake>,
    query: Query<&SnakeBoard>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let board = query.single();
    let snake_head = snake.configuration().next().unwrap();

    if board.collides_with(snake_head)
        || snake.configuration().skip(1).any(|c| c == snake_head)
        || snake.configuration().count() == (board.get_size()*board.get_size()) as usize
    {
        next_state.set(GameState::Menu);
    }
}
    //if board.collides_with(snake_head) {
    //    event.send(GameCompletionEvent(GameOver::HitWall));
    //} else if snake.configuration().any(|c| c == snake_head) {
    //    event.send(GameCompletionEvent(GameOver::HitSnake));
    //} else if snake.configuration().count() == (board.get_size()*board.get_size()) as usize {
    //    event.send(GameCompletionEvent(GameOver::Win));
    //}
