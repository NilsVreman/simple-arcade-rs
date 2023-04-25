use bevy::prelude::{Component, Commands, Query, Entity, With, DespawnRecursiveExt};

/// takes a component as a parameter, and will despawn all entities related to this component
pub fn despawn_component<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
