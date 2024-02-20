use bevy::prelude::{Query, Res, Transform, Vec3};
use bevy::time::Time;

use crate::components::weapons::player_blaster::PlayerBlaster;

pub fn player_blaster_movement(
    mut blasters: Query<(&mut Transform, &PlayerBlaster)>,
    time: Res<Time>,
) {
    for (mut blaster_transform, blaster) in &mut blasters {
        let blaster_speed = blaster.blaster.weapon.velocity * time.delta_seconds();
        let movement_direction = blaster_transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * blaster_speed;
        blaster_transform.translation += translation_delta;
    }
}
