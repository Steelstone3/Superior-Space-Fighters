use bevy::prelude::{Query, Res, Transform, Vec3};
use bevy::time::Time;

use crate::components::blaster::Blaster;

pub fn blaster_movement(mut blasters: Query<(&mut Transform, &Blaster)>, time: Res<Time>) {
    for (mut blaster_transform, blaster) in &mut blasters {
        let blaster_speed = blaster.velocity * time.delta_seconds();
        let movement_direction = blaster_transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * blaster_speed;
        blaster_transform.translation += translation_delta;
    }
}
