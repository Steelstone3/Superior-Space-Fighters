use crate::queries::player_exotic_queries::MutablePlayerExoticTransformQuery;
use bevy::prelude::{Query, Res, Vec3};
use bevy::time::Time;

pub fn player_exotic_movement(
    mut player_exotics: Query<MutablePlayerExoticTransformQuery>,
    time: Res<Time>,
) {
    for mut player_exotic in &mut player_exotics {
        let blaster_speed = player_exotic
            .player_exotic
            .exotic
            .ranged_weapon
            .weapon
            .velocity
            * time.delta_seconds();
        let movement_direction = player_exotic.transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * blaster_speed;
        player_exotic.transform.translation += translation_delta;
    }
}
