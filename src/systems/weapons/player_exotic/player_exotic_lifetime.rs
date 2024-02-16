use bevy::{
    prelude::{Commands, Entity, Query, Res, ResMut},
    time::Time,
    utils::tracing,
};

use crate::{
    components::player_exotic::PlayerExotic, resources::projectile_ammunition::ProjectileAmmunition,
};

pub fn player_exotic_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut exotics: Query<(Entity, &mut PlayerExotic)>,
    mut ammunition: ResMut<ProjectileAmmunition>,
) {
    for (exotic_entity, mut exotic) in &mut exotics {
        exotic.exotic.weapon.lifetime.tick(time.delta());

        if exotic.exotic.weapon.lifetime.finished() {
            commands.entity(exotic_entity).despawn();

            ammunition.exotic_ammunition += 1;

            tracing::info!(
                "Exotic ammunition recovered. Current exotic ammunition: {:?}",
                ammunition.exotic_ammunition
            );
        }
    }
}
