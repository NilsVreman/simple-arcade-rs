use bevy::{
    prelude::{
        Plugin,
        App,
        IntoSystemAppConfig,
        OnEnter,
        OnExit,
        IntoSystemConfig,
        OnUpdate,
        IntoSystemConfigs,
        IntoSystemAppConfigs,
    },
};

use arcade_util::{ArcadeState, despawn_component};

use crate::{
    systems::{
        spawn_popup_window,
        button_system,
        update_text_fields,
    },
    util::PopupWindow,
};

// This file should contain the Result popup plugin
pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                spawn_popup_window
                .in_schedule(OnEnter(ArcadeState::Popup)))
            .add_systems(
                (
                    button_system,
                    update_text_fields,
                )
                .in_set(OnUpdate(ArcadeState::Popup)))
            .add_system(
                despawn_component::<PopupWindow>
                .in_schedule(OnExit(ArcadeState::Popup)));
    }
}
