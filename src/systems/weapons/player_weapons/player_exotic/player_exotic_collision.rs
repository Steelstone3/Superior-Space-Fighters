use crate::{
    components::{
        starships::starship::Starship, weapons::player_weapons::player_exotic::PlayerExotic,
    },
    queries::player_exotic_queries::{MutablePlayerExoticEntityTransformQuery, PlayerExoticFilter},
};
use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    ecs::{query::Without, system::Res},
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

// TODO multi-thread
pub fn player_exotic_collision_with_starship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_exotics: Query<MutablePlayerExoticEntityTransformQuery, PlayerExoticFilter>,
    mut starships: Query<(Entity, &mut Transform, &mut Starship), Without<PlayerExotic>>,
) {
    for mut player_exotic in &mut player_exotics {
        for (starship_entity, starship_transform, mut starship) in &mut starships {
            let distance_to_starship =
                (player_exotic.transform.translation - starship_transform.translation).length();

            let is_collision =
                distance_to_starship < starship.size.x || distance_to_starship < starship.size.y;

            if is_collision {
                tracing::info!("Exotic collision with starship");
                commands.spawn(AudioBundle {
                    source: asset_server
                        .load(player_exotic.player_exotic.exotic.impact_sound.to_string()),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Once,
                        volume: Volume::new(0.2),
                        ..Default::default()
                    },
                });

                player_exotic
                    .player_exotic
                    .exotic
                    .ranged_weapon
                    .weapon
                    .damage
                    .calculate_damage();
                starship.take_damage(
                    player_exotic
                        .player_exotic
                        .exotic
                        .ranged_weapon
                        .weapon
                        .damage,
                );

                tracing::info!(
                    "Enemy Starship | Shield: {:?} | Health: {:?} |",
                    starship.shield.current,
                    starship.hull.current,
                );

                commands.entity(player_exotic.entity).despawn();

                if starship.is_destroyed() {
                    commands.entity(starship_entity).despawn();
                    tracing::info!("Enemy Starship Destroyed");
                }
            }
        }
    }
}
