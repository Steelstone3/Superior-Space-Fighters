use crate::components::weapons::player_weapons::player_torpedo::PlayerTorpedo;
use bevy::{
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

pub fn player_torpedo_lifetime(
    mut commands: Commands,
    mut torpedoes: Query<(Entity, &mut Transform, &mut PlayerTorpedo)>,
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

            tracing::info!("Torpedo despawned",);
        }
    }
}
