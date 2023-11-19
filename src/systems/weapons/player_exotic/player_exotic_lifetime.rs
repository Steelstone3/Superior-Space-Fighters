use crate::{components::exotic::Exotic, resources::exotic_ammunition::ExoticAmmunition};
use bevy::{
    prelude::{Commands, Entity, Query, Res, ResMut},
    time::Time,
    utils::tracing,
};

pub fn player_exotic_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut exotics: Query<(Entity, &mut Exotic)>,
    mut exotic_ammunition: ResMut<ExoticAmmunition>,
) {
    for (exotic_entity, mut exotic) in &mut exotics {
        exotic.lifetime.tick(time.delta());

        if exotic.lifetime.finished() {
            commands.entity(exotic_entity).despawn();

            exotic_ammunition.0 += 1;

            tracing::info!(
                "Exotic ammunition recovered. Current exotic ammunition: {:?}",
                exotic_ammunition.0
            );
        }
    }
}
