use bevy::prelude::Component;

// A struct to create a popup window on the screen printing a message
#[derive(Component)]
pub struct MessageResultPopup;

// An entity for notifying the system that we want to continue
#[derive(Component)]
pub struct ContinueButtonAction;
