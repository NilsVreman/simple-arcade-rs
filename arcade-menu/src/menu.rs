use arcade_util::GameState;
use bevy::{prelude::*, app::AppExit};

use crate::util::{
    MenuState,
    OnMainMenuScreen,
    SelectedOption,
    PRESSED_BUTTON_COLOR,
    HOVERED_PRESSED_BUTTON_COLOR,
    HOVERED_BUTTON_COLOR,
    NORMAL_BUTTON_COLOR,
    TEXT_COLOR,
    MenuButtonAction,
    OnGamesMenuScreen,
    GameListButtonAction,
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
            // entering the `GameState::Menu` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .add_state::<MenuState>()
            // Systems to handle the main menu screen
            .add_system(menu_setup.in_schedule(OnEnter(GameState::Menu)))
            .add_system(main_menu_setup.in_schedule(OnEnter(MenuState::Main)))
            .add_system(despawn_screen::<OnMainMenuScreen>.in_schedule(OnExit(MenuState::Main)))
            // Systems to handle the game list menu screen
            .add_system(game_list_setup.in_schedule(OnEnter(MenuState::GameSelection)))
            .add_system(despawn_screen::<OnGamesMenuScreen>.in_schedule(OnExit(MenuState::GameSelection)))
            // Common systems to all screens that handles buttons behavior
            .add_systems(
                (
                    menu_action,
                    game_list_action,
                    button_system
                )
                .in_set(OnUpdate(GameState::Menu)),
            );
    }
}

// This system handles the buttons background changes
fn button_system(
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

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}


fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_asset = asset_server.load("fonts/FiraSans-Bold.ttf");
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
        ..default()
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
                    ..default()
                })
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );

            // Three buttons: New Game, Game List, Quit
            for (action, text) in [ // Here are all the buttons iterated
                (MenuButtonAction::Play, "New Game"),
                (MenuButtonAction::GameList, "Other Games"),
                (MenuButtonAction::Quit, "Quit"),
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
    let font_asset = asset_server.load("fonts/FiraSans-Bold.ttf");
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
        ..default()
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
                (GameListButtonAction::PlaySnake, "Snake"),
                (GameListButtonAction::BackToMainMenu, "Back"),
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

// Sets state based on the MenuButtonActions
fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(GameState::Snake);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::GameList => menu_state.set(MenuState::GameSelection),
            }
        }
    }
}

// Sets state based on the GameListButtonActions
fn game_list_action(
    interaction_query: Query<(&Interaction, &GameListButtonAction), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, game_list_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match game_list_button_action {
                GameListButtonAction::PlaySnake => {
                    game_state.set(GameState::Snake);
                    menu_state.set(MenuState::Disabled);
                },
                GameListButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
            }
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
