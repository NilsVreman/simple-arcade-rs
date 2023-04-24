use bevy::prelude::States;

// Status
#[derive(States, Default, Debug, Eq, Clone, Hash, PartialEq)]
pub enum GameState {
    Menu,
    #[default]
    PlayingSnake,
}

// Errors
#[derive(Debug)]
pub struct ArcadeError(String);

impl std::error::Error for ArcadeError {}

impl std::fmt::Display for ArcadeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
