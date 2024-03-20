use crate::{
    components::weapons::player_weapons::player_torpedo::PlayerTorpedo,
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{
    prelude::{Commands, Entity, Query, ResMut},
    transform::components::Transform,
    utils::tracing,
};

pub fn player_torpedo_lifetime(
    mut commands: Commands,
    mut torpedoes: Query<(Entity, &mut Transform, &mut PlayerTorpedo)>,
    mut ammunition: ResMut<ProjectileAmmunition>,
) {
    for (torpedo_entity, torpedo_transform, torpedo) in &mut torpedoes {
        let is_past_maximum_range = (torpedo_transform.translation
            - torpedo
                .torpedo
                .lock_on_weapon
                .ranged_weapon
                .original_position)
            .length()
            > torpedo.torpedo.lock_on_weapon.ranged_weapon.range;

        if is_past_maximum_range {
            commands.entity(torpedo_entity).despawn();

            ammunition.torpedo_ammunition += 1;

            tracing::info!(
                "Torpedo ammunition recovered. Current torpedo ammunition: {:?}",
                ammunition.torpedo_ammunition
            );
        }
    }
}
