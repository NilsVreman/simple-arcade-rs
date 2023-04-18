// Status
#[derive(Debug)]
pub enum ArcadeStatus {
    Okay, Fail,
}

#[derive(Debug)]
pub enum GameStatus {
    Running, GameOver,
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
