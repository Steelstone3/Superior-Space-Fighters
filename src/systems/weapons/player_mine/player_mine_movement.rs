use bevy::prelude::{Query, Res, Transform, Vec3};
use bevy::time::Time;

use crate::components::mine::Mine;

pub fn player_mine_movement(mut mines: Query<(&mut Transform, &Mine)>, time: Res<Time>) {
    for (mut mine_transform, mine) in &mut mines {
        let mine_speed = mine.velocity * time.delta_seconds();
        let movement_direction = mine_transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * mine_speed;
        mine_transform.translation += translation_delta;
    }
}
