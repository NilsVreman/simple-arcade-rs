// This file contains all the systems used, in particular for constructing and despawning entities

use bevy::{
    prelude::{
        Commands,
        Res,
        AssetServer,
        default,
        NodeBundle,
        BuildChildren,
        TextBundle,
        ButtonBundle,
        With,
        Button,
        Query,
        ResMut,
        NextState
    },
    text::{TextStyle, Text},
    ui::{
        Style,
        Size,
        Val,
        UiRect,
        JustifyContent,
        AlignItems,
        FlexDirection,
        Interaction,
        BackgroundColor
    }
};

use arcade_util::ArcadeState;

use crate::util::{
    TEXT_COLOR,
    POPUP_COLOR,
    NORMAL_BUTTON_COLOR,
    PRESSED_BUTTON_COLOR,
    HOVERED_BUTTON_COLOR,
    PopupWindow,
    ContinueButtonAction,
    PopupField,
    FieldType,
    PopupMessage,
};

// this function builds a MessageResultPopup entity based on the given Header and Message.
pub fn spawn_popup_window(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font_asset = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .insert(PopupWindow)
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: POPUP_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            // Display the Header
            parent.spawn(TextBundle::from_section(
                "Header Here",
                TextStyle {
                    font_size: 80.0,
                    font: font_asset.clone(),
                    color: TEXT_COLOR,
                })
               .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            )
            .insert(PopupField(FieldType::Header));
            // Display the Message
            parent.spawn(TextBundle::from_section(
                "Message Here",
                TextStyle {
                    font_size: 40.0,
                    font: font_asset.clone(),
                    color: TEXT_COLOR,
                })
               .with_style(Style {
                    margin: UiRect::all(Val::Px(25.0)),
                    ..default()
                }),
            )
            .insert(PopupField(FieldType::Body));
            // Display the Continue Button
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                    margin: UiRect::all(Val::Px(25.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            })
            .insert(ContinueButtonAction)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Continue",
                    TextStyle {
                        font_size: 40.0,
                        color: TEXT_COLOR,
                        font: font_asset.clone(),
                    },
                ));
            });
        });
    });
}

// This system handles the buttons background changes
pub fn button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), With<Button>>,
    mut next_arcade_state: ResMut<NextState<ArcadeState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Clicked => {
                next_arcade_state.set(ArcadeState::Menu);
                PRESSED_BUTTON_COLOR.into()
            },
            Interaction::Hovered => HOVERED_BUTTON_COLOR.into(),
            Interaction::None    => NORMAL_BUTTON_COLOR.into(),
        }
    }
}
// This system updates the Text fields.
pub fn update_text_fields(
    mut textfield_query: Query<(&mut Text, &PopupField)>,
    message: Res<PopupMessage>,
) {
    for (mut text, fieldtype) in &mut textfield_query {
        text.sections[0].value = match fieldtype.0 {
            FieldType::Header => message.get_header().clone(),
            FieldType::Body => message.get_body().clone(),
        };
    }
}
