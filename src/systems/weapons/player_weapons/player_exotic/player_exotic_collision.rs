use bevy::{
    asset::AssetServer,
    audio::AudioBundle,
    ecs::{
        query::Without,
        system::{Res, ResMut},
    },
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

use crate::{
    components::{
        starships::starship::Starship, weapons::player_weapons::player_exotic::PlayerExotic,
    },
    resources::projectile_ammunition::ProjectileAmmunition,
};

// TODO multi-thread
pub fn player_exotic_collision_with_starship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    mut exotics: Query<(Entity, &mut Transform, &mut PlayerExotic), Without<Starship>>,
    mut starships: Query<(Entity, &mut Transform, &mut Starship)>,
) {
    for (exotic_entity, exotic_transform, mut exotic) in &mut exotics {
        for (starship_entity, starship_transform, mut starship) in &mut starships {
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

                exotic.exotic.ranged_weapon.weapon.damage.calculate_damage();
                starship.take_damage(exotic.exotic.ranged_weapon.weapon.damage);

                tracing::info!(
                    "Enemy Starship | Shield: {:?} | Health: {:?} |",
                    starship.shield.current,
                    starship.hull.current,
                );

                commands.entity(exotic_entity).despawn();
                ammunition.exotic_ammunition += 1;

                if starship.is_destroyed() {
                    commands.entity(starship_entity).despawn();
                    tracing::info!("Enemy Starship Destroyed");
                }
            }
        }
    }
}
