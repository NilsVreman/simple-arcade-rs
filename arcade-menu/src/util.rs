use bevy::prelude::{States, Component, Color};

pub const TEXT_COLOR: Color = Color::WHITE;
pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
pub struct OnMainMenuScreen;

// Tag component used to tag entities added on the game list menu screen
#[derive(Component)]
pub struct OnGamesMenuScreen;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    GameSelection,
    #[default]
    Disabled,
}

// All actions that can be triggered from a button click on the main menu
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    GameList,
    Quit,
}

// All actions that can be triggered from a button click on the game list menu
#[derive(Component)]
pub enum GameListButtonAction {
    PlaySnake,
    PlayMinesweeper,
    BackToMainMenu,
}
