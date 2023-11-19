use bevy::{
    prelude::{Commands, Entity, Query, Res, ResMut},
    time::Time,
    utils::tracing,
};

use crate::{
    components::player_blaster::PlayerBlaster, resources::blaster_ammunition::BlasterAmmunition,
};

pub fn player_blaster_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut blasters: Query<(Entity, &mut PlayerBlaster)>,
    mut blaster_ammunition: ResMut<BlasterAmmunition>,
) {
    for (blaster_entity, mut blaster) in &mut blasters {
        blaster.blaster.lifetime.tick(time.delta());

        if blaster.blaster.lifetime.finished() {
            commands.entity(blaster_entity).despawn();

            blaster_ammunition.0 += 1;

            tracing::info!(
                "Blasters ammunition recovered. Current blaster ammunition: {:?}",
                blaster_ammunition.0
            );
        }
    }
}
