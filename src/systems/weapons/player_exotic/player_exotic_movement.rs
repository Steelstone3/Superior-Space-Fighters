use bevy::prelude::{Query, Res, Transform, Vec3};
use bevy::time::Time;

use crate::components::exotic::Exotic;

pub fn player_exotic_movement(mut exotic: Query<(&mut Transform, &Exotic)>, time: Res<Time>) {
    for (mut exotic_transform, exotic) in &mut exotic {
        let blaster_speed = exotic.velocity * time.delta_seconds();
        let movement_direction = exotic_transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * blaster_speed;
        exotic_transform.translation += translation_delta;
    }
}
