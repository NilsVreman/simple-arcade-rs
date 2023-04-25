use arcade_util::{ArcadeState, ActiveGameState};
use bevy::{prelude::*, app::AppExit};

use crate::util::{
    SelectedOption,
    PRESSED_BUTTON_COLOR,
    HOVERED_PRESSED_BUTTON_COLOR,
    HOVERED_BUTTON_COLOR,
    NORMAL_BUTTON_COLOR, MenuState, MenuButtonAction, GameListButtonAction,
};

// This system handles the buttons background changes
pub fn button_system(
    mut interaction_query:
        Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>)
        >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON_COLOR.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON_COLOR.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON_COLOR.into(),
            (Interaction::None, None) => NORMAL_BUTTON_COLOR.into(),
        }
    }
}

// Handles keybindings for the menu
// TODO: Make this more expressive in the future
pub fn keybinding_system(
    input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    menu_state: Res<State<MenuState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut arcade_state: ResMut<NextState<ArcadeState>>,
) {
    if input.just_pressed(KeyCode::Return) {
        next_menu_state.set(MenuState::Disabled);
        arcade_state.set(ArcadeState::Playing);
    } else if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

// Sets state based on the MenuButtonActions
pub fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut arcade_state: ResMut<NextState<ArcadeState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    arcade_state.set(ArcadeState::Playing);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::GameList => menu_state.set(MenuState::GameSelection),
            }
        }
    }
}

// Sets state based on the GameListButtonActions
pub fn game_list_action(
    interaction_query: Query<(&Interaction, &GameListButtonAction), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut arcade_state: ResMut<NextState<ArcadeState>>,
    mut game_state: ResMut<NextState<ActiveGameState>>,
) {
    for (interaction, game_list_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match game_list_button_action {
                GameListButtonAction::PlaySnake => {
                    println!("Switching to Snake");
                    game_state.set(ActiveGameState::Snake);
                },
                GameListButtonAction::PlayMinesweeper => {
                    println!("Switching to Minesweeper");
                    game_state.set(ActiveGameState::Minesweeper);
                },
                GameListButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
            }
        }
    }
}
