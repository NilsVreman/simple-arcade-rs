use std::time::Duration;

// Constants //
pub const TILE_SIZE: f32 = 30.0;
pub const TILE_SPACING: f32 = 1.0;

pub const BOARD_SIZE: i32 = 20;

pub const TICK_DURATION_MS: Duration = Duration::from_millis(125);
pub const MIN_TIMER_DURATION: Duration = Duration::from_millis(50);

// Game completion
pub enum GameOver {
    HitWall, HitSnake, Win
}

pub struct GameCompletionEvent(pub GameOver);
