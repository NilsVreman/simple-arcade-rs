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
        spawn_message_popup,
        button_system,
        update_text_fields,
    },
    util::MessageResultPopup,
};

// This file should contain the Result popup plugin
pub struct MessageResultPlugin;

impl Plugin for MessageResultPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_message_popup.in_schedule(OnEnter(ArcadeState::Result)))
            .add_system(despawn_component::<MessageResultPopup>.in_schedule(OnExit(ArcadeState::Result)))
            .add_systems(
                (
                    button_system,
                    update_text_fields,
                )
                .in_set(OnUpdate(ArcadeState::Result)));
    }
}
