use crate::queries::player_blaster_queries::MutablePlayerBlasterTransformQuery;
use bevy::prelude::{Query, Res, Vec3};
use bevy::time::Time;

pub fn player_blaster_movement(
    mut player_blasters: Query<MutablePlayerBlasterTransformQuery>,
    time: Res<Time>,
) {
    player_blasters
        .par_iter_mut()
        .for_each(|mut player_blaster| {
            let blaster_speed = player_blaster
                .player_blaster
                .blaster
                .ranged_weapon
                .weapon
                .velocity
                * time.delta_seconds();
            let movement_direction = player_blaster.transform.rotation * Vec3::Y;
            let translation_delta = movement_direction * blaster_speed;
            player_blaster.transform.translation += translation_delta;
        });
}
