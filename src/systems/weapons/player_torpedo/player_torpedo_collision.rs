use bevy::{
    asset::AssetServer,
    audio::AudioBundle,
    ecs::{query::Without, system::Res},
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

use crate::components::{starship::Starship, weapons::player_torpedo::PlayerTorpedo};

// TODO multi-thread
pub fn player_torpedo_collision_with_starship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut torpedoes: Query<(Entity, &mut Transform, &mut PlayerTorpedo), Without<Starship>>,
    mut starships: Query<(Entity, &mut Transform, &mut Starship)>,
) {
    for (torpedo_entity, torpedo_transform, torpedo) in &mut torpedoes {
        for (starship_entity, starship_transform, starship) in &mut starships {
            let distance_to_starship =
                (torpedo_transform.translation - starship_transform.translation).length();

            let is_collision =
                distance_to_starship < starship.size.x || distance_to_starship < starship.size.y;

            if is_collision {
                tracing::info!("Torpedo collision with starship");

                commands.spawn(AudioBundle {
                    source: asset_server.load(torpedo.torpedo.impact_sound.to_string()),
                    ..Default::default()
                });

                commands.entity(torpedo_entity).despawn();
                commands.entity(starship_entity).despawn();
            }
        }
    }
}
