use bevy::{
    prelude::*,
    app::AppExit
};

use arcade_util::{ArcadeState, ActiveGameState};

use crate::util::{
    SelectedOption,
    PRESSED_BUTTON_COLOR,
    HOVERED_PRESSED_BUTTON_COLOR,
    HOVERED_BUTTON_COLOR,
    NORMAL_BUTTON_COLOR,
    MenuState,
    MainMenuButtonAction,
    GameMenuButtonAction,
};

type MouseColorInteraction<'a> = (&'a Interaction, &'a mut BackgroundColor, Option<&'a SelectedOption>);
type MouseMainMenuInteraction<'a> = (&'a Interaction, &'a MainMenuButtonAction);
type MouseGameMenuInteraction<'a> = (&'a Interaction, &'a GameMenuButtonAction);

// This system handles the buttons background changes
pub fn button_system(
    mut interaction_query: Query<MouseColorInteraction, With<Button>>,
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
    game_state: Res<State<ActiveGameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_arcade_state: ResMut<NextState<ArcadeState>>,
) {
    if input.just_pressed(KeyCode::Return) {
        next_arcade_state.set(game_state.as_ref().0.as_arcade_state());
        next_menu_state.set(MenuState::Disabled);
    } else if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

// Sets state based on the MenuButtonActions
pub fn menu_action(
    interaction_query: Query<MouseMainMenuInteraction, With<Button>>,
    game_state: Res<State<ActiveGameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_arcade_state: ResMut<NextState<ArcadeState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MainMenuButtonAction::Quit => app_exit_events.send(AppExit),
                MainMenuButtonAction::Play => {
                    next_arcade_state.set(game_state.as_ref().0.as_arcade_state());
                    next_menu_state.set(MenuState::Disabled);
                }
                MainMenuButtonAction::GameList => next_menu_state.set(MenuState::GameSelection),
            }
        }
    }
}

// Sets state based on the GameListButtonActions
pub fn game_list_action(
    interaction_query: Query<MouseGameMenuInteraction, With<Button>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_game_state: ResMut<NextState<ActiveGameState>>,
) {
    for (interaction, game_list_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match game_list_button_action {
                GameMenuButtonAction::PlaySnake => {
                    next_game_state.set(ActiveGameState::Snake);
                },
                GameMenuButtonAction::PlayMinesweeper => {
                    next_game_state.set(ActiveGameState::Minesweeper);
                },
                GameMenuButtonAction::BackToMainMenu => next_menu_state.set(MenuState::Main),
            }
        }
    }
}

// Updates play button text based on active game state
// NOTE: This is a workaround for the lack of dynamic text in bevy. Really convoluted and not
// pretty
pub fn text_update_system(
    game_state: Res<State<ActiveGameState>>,
    mut update_query: Query<(&Button, &Children, &MainMenuButtonAction)>,
    mut children_query: Query<&mut Text>,
) {
    for (_, children, action) in &mut update_query {
        if let MainMenuButtonAction::Play = action {
            for child in children {
                if let Ok(mut text) = children_query.get_mut(*child) {
                    text.sections[0].value = match game_state.as_ref().0 {
                        ActiveGameState::Snake => "Play Snake",
                        ActiveGameState::Minesweeper => "Play Minesweeper",
                    }.to_string();
                }
            }
        }
    }
}
