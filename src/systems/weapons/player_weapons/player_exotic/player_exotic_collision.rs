use crate::queries::{
    player_exotic_queries::{MutablePlayerExoticEntityTransformQuery, PlayerExoticFilter},
    starship_queries::{MutableStarshipEntityTransformQuery, StarshipFilter},
};
use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    ecs::system::Res,
    prelude::{Commands, Query},
    utils::tracing,
};

// TODO multi-thread
pub fn player_exotic_collision_with_starship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_exotics: Query<MutablePlayerExoticEntityTransformQuery, PlayerExoticFilter>,
    mut starships: Query<MutableStarshipEntityTransformQuery, StarshipFilter>,
) {
    for mut player_exotic in &mut player_exotics {
        for mut starship in &mut starships {
            let distance_to_starship =
                (player_exotic.transform.translation - starship.transform.translation).length();

            let is_collision = distance_to_starship < starship.starship.size.x
                || distance_to_starship < starship.starship.size.y;

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
                starship.starship.take_damage(
                    player_exotic
                        .player_exotic
                        .exotic
                        .ranged_weapon
                        .weapon
                        .damage,
                );

                tracing::info!(
                    "Enemy Starship | Shield: {:?} | Health: {:?} |",
                    starship.starship.shield.current,
                    starship.starship.hull.current,
                );

                commands.entity(player_exotic.entity).despawn();
            }
        }
    }
}
