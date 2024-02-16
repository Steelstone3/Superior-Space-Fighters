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
    mut ammunition: ResMut<ProjectileAmmunition>,
) {
    for (blaster_entity, mut blaster) in &mut blasters {
        blaster.blaster.weapon.lifetime.tick(time.delta());

        if blaster.blaster.weapon.lifetime.finished() {
            commands.entity(blaster_entity).despawn();

            ammunition.blaster_ammunition += 1;

            tracing::info!(
                "Blasters ammunition recovered. Current blaster ammunition: {:?}",
                ammunition.blaster_ammunition
            );
        }
    }
}
