use bevy::{
    prelude::*,
    text::TextStyle,
    ui::{
        JustifyContent,
        AlignItems,
        FlexDirection,
        Style,
        Val,
        Size,
        UiRect,
    }
};

use crate::util::{
    OnMainMenuScreen,
    NORMAL_BUTTON_COLOR,
    TEXT_COLOR,
    BACKGROUND_COLOR,
    MainMenuButtonAction,
    OnGamesMenuScreen,
    GameMenuButtonAction,
};

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_asset = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(300.0), Val::Px(65.0)),
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
            background_color: BACKGROUND_COLOR.into(),
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
                (MainMenuButtonAction::Play, "Play Snake"),
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

pub fn game_list_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_asset = asset_server.load("fonts/FiraSans-Bold.ttf");
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
            background_color: BACKGROUND_COLOR.into(),
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
