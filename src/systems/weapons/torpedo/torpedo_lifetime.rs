use bevy::{
    prelude::{Commands, Entity, Query, Res, ResMut},
    time::Time,
    utils::tracing,
};

use crate::{components::torpedo::Torpedo, resources::torpedo_ammunition::TorpedoAmmunition};

pub fn torpedo_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut torpedo: Query<(Entity, &mut Torpedo)>,
    mut torpedo_ammunition: ResMut<TorpedoAmmunition>,
) {
    for (torpedo_entity, mut torpedo) in &mut torpedo {
        torpedo.lifetime.tick(time.delta());

        if torpedo.lifetime.finished() {
            commands.entity(torpedo_entity).despawn();

            torpedo_ammunition.0 += 1;

            tracing::info!(
                "Torpedo ammunition recovered. Current torpedo ammunition: {:?}",
                torpedo_ammunition.0
            );
        }
    }
}
