use bevy::{
    prelude::{Commands, Entity, Query, Res, ResMut},
    time::Time,
    utils::tracing,
};

use crate::{components::player_mine::PlayerMine, resources::mine_ammunition::MineAmmunition};

pub fn player_mine_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut mines: Query<(Entity, &mut PlayerMine)>,
    mut mine_ammunition: ResMut<MineAmmunition>,
) {
    for (mine_entity, mut mine) in &mut mines {
        mine.mine.lifetime.tick(time.delta());

        if mine.mine.lifetime.finished() {
            commands.entity(mine_entity).despawn();

            mine_ammunition.0 += 1;

            tracing::info!("Mine recovered. Current mines: {:?}", mine_ammunition.0);
        }
    }
}
