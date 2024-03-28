use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    ecs::{query::Without, system::Res},
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

use crate::{
    components::{
        starships::starship::Starship, weapons::player_weapons::player_blaster::PlayerBlaster,
    },
    queries::player_blaster_queries::{
        MutablePlayerBlasterEntityTransformQuery, PlayerBlasterFilter,
    },
};

// TODO multi-thread
pub fn player_blaster_collision_with_starship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_blasters: Query<MutablePlayerBlasterEntityTransformQuery, PlayerBlasterFilter>,
    mut starships: Query<(Entity, &mut Transform, &mut Starship), Without<PlayerBlaster>>,
) {
    for mut player_blaster in &mut player_blasters {
        for (starship_entity, starship_transform, mut starship) in &mut starships {
            let distance_to_starship =
                (player_blaster.transform.translation - starship_transform.translation).length();

            let is_collision =
                distance_to_starship < starship.size.x || distance_to_starship < starship.size.y;

            if is_collision {
                tracing::info!("Blaster collision with starship");
                commands.spawn(AudioBundle {
                    source: asset_server.load(
                        player_blaster
                            .player_blaster
                            .blaster
                            .impact_sound
                            .to_string(),
                    ),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Once,
                        volume: Volume::new(0.2),
                        ..Default::default()
                    },
                });

                player_blaster
                    .player_blaster
                    .blaster
                    .ranged_weapon
                    .weapon
                    .damage
                    .calculate_damage();
                starship.take_damage(
                    player_blaster
                        .player_blaster
                        .blaster
                        .ranged_weapon
                        .weapon
                        .damage,
                );

                tracing::info!(
                    "Enemy Starship | Shield: {:?} | Health: {:?} |",
                    starship.shield.current,
                    starship.hull.current,
                );

                commands.entity(player_blaster.entity).despawn();

                if starship.is_destroyed() {
                    commands.entity(starship_entity).despawn();
                    tracing::info!("Enemy Starship Destroyed");
                }
            }
        }
    }
}
