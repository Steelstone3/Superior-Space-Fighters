use crate::components::torpedo::Torpedo;
use bevy::prelude::{Query, Res, Transform, Vec3};
use bevy::time::Time;

pub fn torpedo_movement(mut torpedos: Query<(&mut Transform, &Torpedo)>, time: Res<Time>) {
    for (mut torpedo_transform, torpedo) in &mut torpedos {
        let blaster_speed = torpedo.velocity * time.delta_seconds();
        let movement_direction = torpedo_transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * blaster_speed;
        torpedo_transform.translation += translation_delta;
    }
}
