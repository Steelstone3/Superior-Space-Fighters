use crate::components::weapons::player_weapons::player_exotic::PlayerExotic;
use bevy::{
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

pub fn player_exotic_lifetime(
    mut commands: Commands,
    mut exotics: Query<(Entity, &mut Transform, &mut PlayerExotic)>,
) {
    for (exotic_entity, exotic_transform, exotic) in &mut exotics {
        let is_past_maximum_range =
            (exotic_transform.translation - exotic.exotic.ranged_weapon.original_position).length()
                > exotic.exotic.ranged_weapon.range;

        if is_past_maximum_range {
            commands.entity(exotic_entity).despawn();

            tracing::info!("Exotic despawned",);
        }
    }
}
