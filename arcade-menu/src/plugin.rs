use bevy::prelude::{
    Plugin,
    App,
    OnEnter,
    ResMut,
    NextState,
    IntoSystemAppConfig,
    OnExit,
    IntoSystemConfigs,
    OnUpdate
};

use arcade_util::{ActiveGameState, ArcadeState, despawn_component};

use crate::{
    util::{MenuState, OnMainMenuScreen, OnGamesMenuScreen},
    menu::{main_menu_setup, game_list_setup},
    systems::{menu_action, game_list_action, button_system, keybinding_system, text_update_system}
};

// This plugin manages the menu, with 2 different screens:
// - a main menu with "Play *ActiveGamEState*", "More Games", "Quit"
// - A game list menu with a list of games to choose from
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `ArcadeState::Menu` state.
            // Current screen in the menu is handled by an independent state from `ArcadeState`
            .add_state::<MenuState>()
            .add_state::<ActiveGameState>()
            // Systems to handle the main menu screen
            .add_system(menu_setup.in_schedule(OnEnter(ArcadeState::Menu)))
            .add_system(main_menu_setup.in_schedule(OnEnter(MenuState::Main)))
            .add_system(despawn_component::<OnMainMenuScreen>.in_schedule(OnExit(MenuState::Main)))
            // Systems to handle the game list menu screen
            .add_system(game_list_setup.in_schedule(OnEnter(MenuState::GameSelection)))
            .add_system(despawn_component::<OnGamesMenuScreen>.in_schedule(OnExit(MenuState::GameSelection)))
            // Common systems to all screens that handles buttons behavior
            .add_systems(
                (
                    menu_action,
                    game_list_action,
                    button_system,
                    keybinding_system,
                    text_update_system,
                )
                .in_set(OnUpdate(ArcadeState::Menu)),
            );
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}
