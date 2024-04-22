use bevy::{
    ecs::system::{Commands, Query},
    hierarchy::DespawnRecursiveExt,
};

use crate::queries::{entity_query::EntityQuery, game_objects_query::InGameEntityFilter};

pub fn despawn_on_return_to_menu(
    entities: Query<EntityQuery, InGameEntityFilter>,
    mut commands: Commands,
) {
    for entity in entities.iter() {
        commands.entity(entity.entity).despawn_recursive();
    }
}
