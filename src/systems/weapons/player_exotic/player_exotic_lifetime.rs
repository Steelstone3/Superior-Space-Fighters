use crate::{
    components::weapons::player_exotic::PlayerExotic,
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{
    prelude::{Commands, Entity, Query, ResMut},
    transform::components::Transform,
    utils::tracing,
};

pub fn player_exotic_lifetime(
    mut commands: Commands,
    mut exotics: Query<(Entity, &mut Transform, &mut PlayerExotic)>,
    mut ammunition: ResMut<ProjectileAmmunition>,
) {
    for (exotic_entity, exotic_transform, exotic) in &mut exotics {
        let is_past_maximum_range = (exotic_transform.translation
            - exotic.exotic.ranged_weapon.weapon.original_position)
            .length()
            > exotic.exotic.ranged_weapon.range;

        if is_past_maximum_range {
            commands.entity(exotic_entity).despawn();

            ammunition.exotic_ammunition += 1;

            tracing::info!(
                "Exotic ammunition recovered. Current exotic ammunition: {:?}",
                ammunition.exotic_ammunition
            );
        }
    }
}
