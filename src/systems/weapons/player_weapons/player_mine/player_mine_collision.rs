use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    ecs::{query::Without, system::Res},
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

use crate::components::{
    starships::starship::Starship, weapons::player_weapons::player_mine::PlayerMine,
};

// TODO multi-thread
pub fn player_mine_collision_with_starship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut mines: Query<(Entity, &mut Transform, &mut PlayerMine), Without<Starship>>,
    mut starships: Query<(Entity, &mut Transform, &mut Starship), Without<PlayerMine>>,
) {
    for (mine_entity, mine_transform, mut mine) in &mut mines {
        for (starship_entity, starship_transform, mut starship) in &mut starships {
            let distance_to_starship =
                (mine_transform.translation - starship_transform.translation).length();

            let is_collision =
                distance_to_starship < starship.size.x || distance_to_starship < starship.size.y;

            if is_collision {
                tracing::info!("Mine collision with starship");
                commands.spawn(AudioBundle {
                    source: asset_server.load(mine.mine.impact_sound.to_string()),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Once,
                        volume: Volume::new(0.5),
                        ..Default::default()
                    },
                });

                mine.mine.lifetime_weapon.weapon.damage.calculate_damage();
                starship.take_damage(mine.mine.lifetime_weapon.weapon.damage);

                tracing::info!(
                    "Enemy Starship | Shield: {:?} | Health: {:?} |",
                    starship.shield.current,
                    starship.hull.current,
                );

                commands.entity(mine_entity).despawn();

                if starship.is_destroyed() {
                    commands.entity(starship_entity).despawn();
                    tracing::info!("Enemy Starship Destroyed");
                }
            }
        }
    }
}
