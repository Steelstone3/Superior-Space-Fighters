use bevy::prelude::{Query, Res, Vec3};
use bevy::time::Time;

use crate::queries::player_mine_queries::MutablePlayerMineTransformQuery;

pub fn player_mine_movement(
    mut player_mines: Query<MutablePlayerMineTransformQuery>,
    time: Res<Time>,
) {
    player_mines.par_iter_mut().for_each(|mut player_mine| {
        let mine_speed =
            player_mine.player_mine.mine.lifetime_weapon.weapon.velocity * time.delta_seconds();
        let movement_direction = player_mine.transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * mine_speed;
        player_mine.transform.translation += translation_delta;
    });
}
