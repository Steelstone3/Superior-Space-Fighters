use crate::{
    components::weapons::player_mine::PlayerMine,
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{
    prelude::{Commands, Entity, Query, Res, ResMut},
    time::Time,
    utils::tracing,
};

pub fn player_mine_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut mines: Query<(Entity, &mut PlayerMine)>,
    mut ammunition: ResMut<ProjectileAmmunition>,
) {
    for (mine_entity, mut mine) in &mut mines {
        mine.mine.lifetime_weapon.lifetime.tick(time.delta());

        if mine.mine.lifetime_weapon.lifetime.finished() {
            commands.entity(mine_entity).despawn();

            ammunition.mine_ammunition += 1;

            tracing::info!(
                "Mine recovered. Current mines: {:?}",
                ammunition.mine_ammunition
            );
        }
    }
}
