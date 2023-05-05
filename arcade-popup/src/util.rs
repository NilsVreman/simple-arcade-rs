use bevy::{
    prelude::{Color, Resource, World, Query, Component, Commands},
    ecs::system::Command
};

pub const TEXT_COLOR: Color = Color::WHITE;
pub const POPUP_COLOR: Color = Color::CRIMSON;
pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

// This is the resource that will be added to the popup plugin to print the correct messages
#[derive(Resource, Clone)]
pub struct PopupMessage {
    header: String,
    body: String,
}

impl PopupMessage {
    pub fn get_header(&self) -> &String { &self.header }
    pub fn get_body(&self) -> &String { &self.body }
}

impl Default for PopupMessage {
    fn default() -> Self {
        Self {
            header: String::from("Header"),
            body: String::from("Body"),
        }
    }
}

#[derive(Component, Clone)]
pub struct ChangePopupMessage {
    header: String,
    body: String,
}

impl Command for ChangePopupMessage {
    fn write(self, world: &mut World) {
        let mut popup = world.get_resource_or_insert_with(PopupMessage::default);
        popup.header = self.header;
        popup.body = self.body;
    }
}

pub fn spawn_popup(
    mut commands: Commands,
    header: String,
    body: String,
) {
    let message = ChangePopupMessage { header, body, };
    commands.add(message.clone());
}

// A struct to create a popup window on the screen printing a message
#[derive(Component)]
pub struct PopupWindow;

// An entity for notifying the system that we want to continue
#[derive(Component)]
pub struct ContinueButtonAction;

// An entity for notifying that a field is a textfield
#[derive(Component)]
pub struct PopupField(pub FieldType);

pub enum FieldType {
    Header,
    Body,
}
