use bevy::prelude::{Query, Res, Transform, Vec3};
use bevy::time::Time;

use crate::components::weapons::player_torpedo::PlayerTorpedo;

pub fn player_torpedo_movement(
    mut torpedos: Query<(&mut Transform, &PlayerTorpedo)>,
    time: Res<Time>,
) {
    for (mut torpedo_transform, torpedo) in &mut torpedos {
        let blaster_speed = torpedo.torpedo.weapon.velocity * time.delta_seconds();
        let movement_direction = torpedo_transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * blaster_speed;
        torpedo_transform.translation += translation_delta;
    }
}
