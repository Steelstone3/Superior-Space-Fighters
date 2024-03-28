use crate::queries::player_torpedo_queries::MutablePlayerTorpedoEntityTransformQuery;
use bevy::{
    prelude::{Commands, Query},
    utils::tracing,
};

pub fn player_torpedo_lifetime(
    mut commands: Commands,
    mut player_torpedoes: Query<MutablePlayerTorpedoEntityTransformQuery>,
) {
    for player_torpedo in &mut player_torpedoes {
        let is_past_maximum_range = (player_torpedo.transform.translation
            - player_torpedo
                .player_torpedo
                .torpedo
                .lock_on_weapon
                .ranged_weapon
                .original_position)
            .length()
            > player_torpedo
                .player_torpedo
                .torpedo
                .lock_on_weapon
                .ranged_weapon
                .range;

        if is_past_maximum_range {
            commands.entity(player_torpedo.entity).despawn();

            tracing::info!("Torpedo despawned",);
        }
    }
}
