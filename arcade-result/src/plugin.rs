use bevy::prelude::{
    Plugin,
    App,
    IntoSystemAppConfig,
    OnEnter, OnExit, IntoSystemConfig, OnUpdate
};

use arcade_util::{ArcadeState, despawn_component};

use crate::systems::{spawn_message_popup, button_system};
use crate::components::MessageResultPopup;

// This file should contain the Result popup plugin
pub struct MessageResultPlugin;

impl Plugin for MessageResultPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_message_popup.in_schedule(OnEnter(ArcadeState::Result)))
            .add_system(despawn_component::<MessageResultPopup>.in_schedule(OnExit(ArcadeState::Result)))
            .add_system(button_system.in_set(OnUpdate(ArcadeState::Result)));
    }
}
