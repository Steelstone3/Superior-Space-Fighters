use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    ecs::system::Res,
    prelude::{Commands, Query},
    utils::tracing,
};

use crate::queries::{
    player_torpedo_queries::{MutablePlayerTorpedoEntityTransformQuery, PlayerTorpedoFilter},
    starship_queries::{MutableStarshipEntityTransformQuery, StarshipFilter},
};

// TODO multi-thread
pub fn player_torpedo_collision_with_starship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_torpedoes: Query<MutablePlayerTorpedoEntityTransformQuery, PlayerTorpedoFilter>,
    mut starships: Query<MutableStarshipEntityTransformQuery, StarshipFilter>,
) {
    for mut player_torpedo in &mut player_torpedoes {
        for mut starship in &mut starships {
            let distance_to_starship =
                (player_torpedo.transform.translation - starship.transform.translation).length();

            let is_collision = distance_to_starship < starship.starship.size.x
                || distance_to_starship < starship.starship.size.y;

            if is_collision {
                tracing::info!("Torpedo collision with starship");
                commands.spawn(AudioBundle {
                    source: asset_server.load(
                        player_torpedo
                            .player_torpedo
                            .torpedo
                            .impact_sound
                            .to_string(),
                    ),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Once,
                        volume: Volume::new(0.2),
                        ..Default::default()
                    },
                });

                player_torpedo
                    .player_torpedo
                    .torpedo
                    .lock_on_weapon
                    .ranged_weapon
                    .weapon
                    .damage
                    .calculate_damage();
                starship.starship.take_damage(
                    player_torpedo
                        .player_torpedo
                        .torpedo
                        .lock_on_weapon
                        .ranged_weapon
                        .weapon
                        .damage,
                );

                tracing::info!(
                    "Enemy Starship | Shield: {:?} | Health: {:?} |",
                    starship.starship.shield.current,
                    starship.starship.hull.current,
                );

                commands.entity(player_torpedo.entity).despawn();
            }
        }
    }
}
