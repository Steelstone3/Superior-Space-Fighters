use bevy::prelude::{Query, Res, Transform, Vec3};
use bevy::time::Time;

use crate::components::weapons::player_mine::PlayerMine;

pub fn player_mine_movement(mut mines: Query<(&mut Transform, &PlayerMine)>, time: Res<Time>) {
    for (mut mine_transform, mine) in &mut mines {
        let mine_speed = mine.mine.weapon.velocity * time.delta_seconds();
        let movement_direction = mine_transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * mine_speed;
        mine_transform.translation += translation_delta;
    }
}
