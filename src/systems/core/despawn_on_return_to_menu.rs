use bevy::{
    ecs::{
        query::{Or, With},
        system::{Commands, Query},
    },
    hierarchy::DespawnRecursiveExt,
    ui::Node,
};

use crate::{
    components::{
        space::Space,
        starships::starship::Starship,
        station::SpaceStation,
        weapons::player_weapons::{
            player_blaster::PlayerBlaster, player_exotic::PlayerExotic, player_mine::PlayerMine,
            player_torpedo::PlayerTorpedo,
        },
    },
    queries::entity_query::EntityQuery,
};

pub fn despawn_on_return_to_menu(
    entities: Query<
        EntityQuery,
        Or<(
            With<Space>,
            With<SpaceStation>,
            With<Starship>,
            With<Node>,
            With<PlayerBlaster>,
            With<PlayerMine>,
            With<PlayerTorpedo>,
            With<PlayerExotic>,
        )>,
    >,
    mut commands: Commands,
) {
    for entity in entities.iter() {
        commands.entity(entity.entity).despawn_recursive();
    }
}
