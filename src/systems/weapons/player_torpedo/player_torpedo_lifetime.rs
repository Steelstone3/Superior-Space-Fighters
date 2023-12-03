use crate::{
    components::player_torpedo::PlayerTorpedo,
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{
    prelude::{Commands, Entity, Query, Res, ResMut},
    time::Time,
    utils::tracing,
};

pub fn player_torpedo_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut torpedo: Query<(Entity, &mut PlayerTorpedo)>,
    mut ammunition: ResMut<ProjectileAmmunition>,
) {
    for (torpedo_entity, mut torpedo) in &mut torpedo {
        torpedo.torpedo.lifetime.tick(time.delta());

        if torpedo.torpedo.lifetime.finished() {
            commands.entity(torpedo_entity).despawn();

            ammunition.torpedo_ammunition += 1;

            tracing::info!(
                "Torpedo ammunition recovered. Current torpedo ammunition: {:?}",
                ammunition.torpedo_ammunition
            );
        }
    }
}
