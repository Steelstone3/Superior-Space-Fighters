use crate::components::weapons::player_weapons::player_blaster::PlayerBlaster;
use bevy::{
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

pub fn player_blaster_lifetime(
    mut commands: Commands,
    mut blasters: Query<(Entity, &mut Transform, &mut PlayerBlaster)>,
) {
    for (blaster_entity, blaster_transform, blaster) in &mut blasters {
        let is_past_maximum_range = (blaster_transform.translation
            - blaster.blaster.ranged_weapon.original_position)
            .length()
            > blaster.blaster.ranged_weapon.range;

        if is_past_maximum_range {
            commands.entity(blaster_entity).despawn();

            tracing::info!("Blasters despawned",);
        }
    }
}
