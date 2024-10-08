use crate::queries::player_exotic_queries::PlayerExoticEntityTransformQuery;
use bevy::{
    prelude::{Commands, Query},
    utils::tracing,
};

pub fn player_exotic_lifetime(
    mut commands: Commands,
    player_exotics: Query<PlayerExoticEntityTransformQuery>,
) {
    for player_exotic in &player_exotics {
        let is_past_maximum_range = (player_exotic.transform.translation
            - player_exotic
                .player_exotic
                .exotic
                .ranged_weapon
                .original_position)
            .length()
            > player_exotic.player_exotic.exotic.ranged_weapon.range;

        if is_past_maximum_range {
            commands.entity(player_exotic.entity).despawn();

            tracing::info!("Exotic despawned",);
        }
    }
}
