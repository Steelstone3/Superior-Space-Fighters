use crate::{
    components::weapons::player_weapons::player_blaster::PlayerBlaster,
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{
    prelude::{Commands, Entity, Query, ResMut},
    transform::components::Transform,
    utils::tracing,
};

pub fn player_blaster_lifetime(
    mut commands: Commands,
    mut blasters: Query<(Entity, &mut Transform, &mut PlayerBlaster)>,
    mut ammunition: ResMut<ProjectileAmmunition>,
) {
    for (blaster_entity, blaster_transform, blaster) in &mut blasters {
        let is_past_maximum_range = (blaster_transform.translation
            - blaster.blaster.ranged_weapon.original_position)
            .length()
            > blaster.blaster.ranged_weapon.range;

        if is_past_maximum_range {
            commands.entity(blaster_entity).despawn();

            ammunition.blaster_ammunition += 1;

            tracing::info!(
                "Blasters ammunition recovered. Current blaster ammunition: {:?}",
                ammunition.blaster_ammunition
            );
        }
    }
}
