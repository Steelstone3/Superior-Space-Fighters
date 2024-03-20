use crate::components::weapons::player_weapons::player_mine::PlayerMine;
use bevy::{
    prelude::{Commands, Entity, Query, Res},
    time::Time,
    utils::tracing,
};

pub fn player_mine_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut mines: Query<(Entity, &mut PlayerMine)>,
) {
    for (mine_entity, mut mine) in &mut mines {
        mine.mine.lifetime_weapon.lifetime.tick(time.delta());

        if mine.mine.lifetime_weapon.lifetime.finished() {
            commands.entity(mine_entity).despawn();

            tracing::info!("Mine despawn",);
        }
    }
}
