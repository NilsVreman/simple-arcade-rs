use bevy::prelude::*;
use bevy::text::{TextStyle, Font};
use bevy::ui::{
    JustifyContent,
    AlignItems,
    FlexDirection,
    Style,
    Val,
    Size,
    UiRect,
};

use arcade_util::{
    ArcadeState, 
    ActiveGameState,
    despawn_component, 
};

use crate::util::{
    MenuState,
    OnMainMenuScreen,
    NORMAL_BUTTON_COLOR,
    TEXT_COLOR,
    MainMenuButtonAction,
    OnGamesMenuScreen,
    GameMenuButtonAction,
};
use crate::systems::{
    menu_action,
    game_list_action,
    button_system,
    keybinding_system,
};

// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
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
                )
                .in_set(OnUpdate(ArcadeState::Menu)),
            );
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn load_font(asset_server: Res<AssetServer>) -> Handle<Font> {
    asset_server.load("fonts/FiraSans-Bold.ttf")
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_asset = load_font(asset_server);
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        font: font_asset.clone(),
    };

    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .insert(OnMainMenuScreen)
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::CRIMSON.into(),
            ..default()
        })
        .with_children(|parent| {
            // Display the game name
            parent.spawn(TextBundle::from_section(
                "Main Menu",
                TextStyle {
                    font_size: 80.0,
                    font: font_asset.clone(),
                    color: TEXT_COLOR,
                })
               .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );

            // Three buttons: New Game, Game List, Quit
            for (action, text) in [ // Here are all the buttons iterated
                (MainMenuButtonAction::Play, "New Game"),
                (MainMenuButtonAction::GameList, "Other Games"),
                (MainMenuButtonAction::Quit, "Quit"),
            ] {
                parent.spawn(ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(action)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        text,
                        button_text_style.clone(),
                    ));
                });
            }
        });
    });
}

fn game_list_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_asset = load_font(asset_server);
    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        font: font_asset,
    };

    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .insert(OnGamesMenuScreen)
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::CRIMSON.into(),
            ..default()
        })
        .with_children(|parent| {
            // X buttons: Snake, Back
            for (action, text) in [ // Here are all the buttons iterated
                (GameMenuButtonAction::PlaySnake, "Snake"),
                (GameMenuButtonAction::PlayMinesweeper, "Minesweeper"),
                (GameMenuButtonAction::BackToMainMenu, "Back"),
            ] {
                parent.spawn(ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(action)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        text,
                        button_text_style.clone(),
                    ));
                });
            }
        });
    });
}
