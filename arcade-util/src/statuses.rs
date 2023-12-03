use bevy::prelude::States;

// Status
#[derive(States, Default, Debug, Eq, Clone, Hash, PartialEq)]
pub enum ArcadeState {
    #[default]
    Menu,
    Popup,
    PlayingSnake,
    PlayingMinesweeper,
    PlayingPacman,
}

// Status
#[derive(States, Default, Debug, Eq, Clone, Hash, PartialEq)]
pub enum ActiveGameState {
    #[default]
    Snake,
    Minesweeper,
    Pacman,
}

impl ActiveGameState {
    pub fn as_arcade_state(&self) -> ArcadeState {
        match self {
            ActiveGameState::Snake       => ArcadeState::PlayingSnake,
            ActiveGameState::Minesweeper => ArcadeState::PlayingMinesweeper,
            ActiveGameState::Pacman      => ArcadeState::PlayingPacman,
        }
    }
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
