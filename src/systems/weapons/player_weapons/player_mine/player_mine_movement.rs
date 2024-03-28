use bevy::prelude::{Query, Res, Vec3};
use bevy::time::Time;

use crate::queries::player_mine_queries::MutablePlayerMineTransformQuery;

pub fn player_mine_movement(
    mut player_mines: Query<MutablePlayerMineTransformQuery>,
    time: Res<Time>,
) {
    for mut player_mine in &mut player_mines {
        let mine_speed =
            player_mine.player_mine.mine.lifetime_weapon.weapon.velocity * time.delta_seconds();
        let movement_direction = player_mine.transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * mine_speed;
        player_mine.transform.translation += translation_delta;
    }
}
