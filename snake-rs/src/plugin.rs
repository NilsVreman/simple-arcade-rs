use std::{time::Duration};
use arcade_util::Coord2D;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const LEFT_WALL_PX: f32        = -400.;
const RIGHT_WALL_PX: f32       = 400.;
const BOTTOM_WALL_PX: f32      = -400.;
const TOP_WALL_PX: f32         = 400.;
const ARENA_WIDTH: f32         = 10.0;
const ARENA_HEIGHT: f32        = 10.0;

const SEGMENT_X_PX: f32        = (RIGHT_WALL_PX - LEFT_WALL_PX) / ARENA_WIDTH;
const SEGMENT_Y_PX: f32        = (TOP_WALL_PX - BOTTOM_WALL_PX) / ARENA_HEIGHT;
const SEGMENT: Vec2            = Vec2::new(SEGMENT_X_PX, SEGMENT_Y_PX);
const SPACING: Vec2            = Vec2::new(10.0, 0.0);
const WALL_THICKNESS_X_PX: f32 = SEGMENT_X_PX;
const WALL_THICKNESS_Y_PX: f32 = SEGMENT_Y_PX;

const MIN_TIMER_DURATION: Duration = Duration::from_millis(125);
const WALL_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);


use crate::{SnakeState};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }))
            .add_startup_systems( (
                setup_general,
                setup_walls,
                setup_snake,
                ))
            .add_system(tick_game)
            .add_system(snake_movement);
    }
}

// Bundle //

enum WallLocation {
    Top, Bottom, Left, Right,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        todo!("Fix grid size!");
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL_PX, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL_PX, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL_PX),
            WallLocation::Top => Vec2::new(0., TOP_WALL_PX),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL_PX - BOTTOM_WALL_PX;
        let arena_width = RIGHT_WALL_PX - LEFT_WALL_PX;

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS_X_PX, arena_height + WALL_THICKNESS_Y_PX)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS_X_PX, WALL_THICKNESS_Y_PX)
            }
        }
    }
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
}

impl WallBundle {
    fn new(location: WallLocation) -> Self {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate This is
                    // used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0, or their ordering will be
                    // affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            }
        }
    }
}

#[derive(Component)]
struct SnakeSegment;

// Resources //

#[derive(Component)]
struct SnakeTimer(Timer);

impl SnakeTimer {
    fn update_timer(&mut self) {
        if self.0.duration() > MIN_TIMER_DURATION {
            self.0 = Timer::new(self.0.duration() / 2, TimerMode::Repeating);
        }
    }
}

// Systems //

fn setup_general(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());  // Camera
}

fn setup_walls(mut commands: Commands) {
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
}

fn setup_snake(mut commands: Commands) {
    // add the head of the snake
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: SEGMENT.extend(1.0),
            ..default()
        },
        sprite: Sprite {
            color: SNAKE_COLOR,
            ..default()
        },
        ..default()
    })
    .insert(SnakeSegment);
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head: Query<(&SnakeSegment, &mut Transform)>
) {
    if let Some((_, mut transform)) = head.iter_mut().next() {
        if keyboard_input.just_pressed(KeyCode::H)
            || keyboard_input.just_pressed(KeyCode::Left)
        {
            transform.translation.x -= SEGMENT_X_PX;
        } else if keyboard_input.just_pressed(KeyCode::J)
            || keyboard_input.just_pressed(KeyCode::Down)
        {
            transform.translation.y -= SEGMENT_X_PX;
        } else if keyboard_input.just_pressed(KeyCode::K)
            || keyboard_input.just_pressed(KeyCode::Up)
        {
            transform.translation.y += SEGMENT_X_PX;
        } else if keyboard_input.just_pressed(KeyCode::L)
            || keyboard_input.just_pressed(KeyCode::Right)
        {
            transform.translation.x += SEGMENT_X_PX;
        }
    }
}

fn tick_game(
    time: Res<Time>,
    mut query: Query<(&mut SnakeState, &mut SnakeTimer)>
    ) {
    for (game, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            timer.update_timer();
            println!("{:?}", timer.0);
            game.draw();
        }
    }
}
