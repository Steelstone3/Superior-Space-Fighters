use crate::components::{player_blaster::PlayerBlaster, starship::Starship};
use bevy::{
    asset::AssetServer,
    audio::AudioBundle,
    ecs::{query::Without, system::Res},
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

// TODO multi-thread
pub fn player_blaster_collision_with_starship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut blasters: Query<(Entity, &mut Transform, &mut PlayerBlaster), Without<Starship>>,
    mut starships: Query<(Entity, &mut Transform, &mut Starship)>,
) {
    for (blaster_entity, blaster_transform, blaster) in &mut blasters {
        for (starship_entity, starship_transform, starship) in &mut starships {
            let distance_to_starship =
                (blaster_transform.translation - starship_transform.translation).length();

            let is_collision =
                distance_to_starship < starship.size.x || distance_to_starship < starship.size.y;

            if is_collision {
                tracing::info!("Blaster collision with starship");

                commands.spawn(AudioBundle {
                    source: asset_server.load(blaster.blaster.impact_sound.to_string()),
                    ..Default::default()
                });

                commands.entity(blaster_entity).despawn();
                commands.entity(starship_entity).despawn();
            }
        }
    }
}
