use crate::queries::player_blaster_queries::MutablePlayerBlasterEntityTransformQuery;
use bevy::{
    prelude::{Commands, Query},
    utils::tracing,
};

pub fn player_blaster_lifetime(
    mut commands: Commands,
    mut player_blasters: Query<MutablePlayerBlasterEntityTransformQuery>,
) {
    for player_blaster in &mut player_blasters {
        let is_past_maximum_range = (player_blaster.transform.translation
            - player_blaster
                .player_blaster
                .blaster
                .ranged_weapon
                .original_position)
            .length()
            > player_blaster.player_blaster.blaster.ranged_weapon.range;

        if is_past_maximum_range {
            commands.entity(player_blaster.entity).despawn();

            tracing::info!("Blasters despawned",);
        }
    }
}
