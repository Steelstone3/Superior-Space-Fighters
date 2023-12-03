use crate::{
    components::player_blaster::PlayerBlaster,
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{
    prelude::{Commands, Entity, Query, Res, ResMut},
    time::Time,
    utils::tracing,
};

pub fn player_blaster_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut blasters: Query<(Entity, &mut PlayerBlaster)>,
    mut blaster_ammunition: ResMut<ProjectileAmmunition>,
) {
    for (blaster_entity, mut blaster) in &mut blasters {
        blaster.blaster.lifetime.tick(time.delta());

        if blaster.blaster.lifetime.finished() {
            commands.entity(blaster_entity).despawn();

            blaster_ammunition.blaster_ammunition += 1;

            tracing::info!(
                "Blasters ammunition recovered. Current blaster ammunition: {:?}",
                blaster_ammunition.blaster_ammunition
            );
        }
    }
}
