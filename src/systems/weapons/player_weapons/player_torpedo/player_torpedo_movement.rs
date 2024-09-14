use bevy::prelude::{Query, Res, Vec3};
use bevy::time::Time;

use crate::queries::player_torpedo_queries::MutablePlayerTorpedoTransformQuery;

pub fn player_torpedo_movement(
    mut player_torpedoes: Query<MutablePlayerTorpedoTransformQuery>,
    time: Res<Time>,
) {
    player_torpedoes
        .par_iter_mut()
        .for_each(|mut player_torpedo| {
            let blaster_speed = player_torpedo
                .player_torpedo
                .torpedo
                .lock_on_weapon
                .ranged_weapon
                .weapon
                .velocity
                * time.delta_seconds();
            let movement_direction = player_torpedo.transform.rotation * Vec3::Y;
            let translation_delta = movement_direction * blaster_speed;
            player_torpedo.transform.translation += translation_delta;
        });
}
