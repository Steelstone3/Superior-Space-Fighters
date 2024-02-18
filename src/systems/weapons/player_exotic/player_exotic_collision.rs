use crate::components::{player_exotic::PlayerExotic, starship::Starship};
use bevy::{
    asset::AssetServer,
    audio::AudioBundle,
    ecs::{query::Without, system::Res},
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

// TODO multi-thread
pub fn player_exotic_collision_with_starship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut exotics: Query<(Entity, &mut Transform, &mut PlayerExotic), Without<Starship>>,
    mut starships: Query<(Entity, &mut Transform, &mut Starship)>,
) {
    for (exotic_entity, exotic_transform, exotic) in &mut exotics {
        for (starship_entity, starship_transform, starship) in &mut starships {
            let distance_to_starship =
                (exotic_transform.translation - starship_transform.translation).length();

            let is_collision =
                distance_to_starship < starship.size.x || distance_to_starship < starship.size.y;

            if is_collision {
                tracing::info!("Exotic collision with starship");

                commands.spawn(AudioBundle {
                    source: asset_server.load(exotic.exotic.impact_sound.to_string()),
                    ..Default::default()
                });

                commands.entity(exotic_entity).despawn();
                commands.entity(starship_entity).despawn();
            }
        }
    }
}
