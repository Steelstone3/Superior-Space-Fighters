use crate::resources::{
    projectile_ammunition::ProjectileAmmunition,
    projectile_ammunition_maximum::ProjectileAmmunitionMaximum,
};
use bevy::{
    prelude::{Res, ResMut},
    time::Time,
};

pub fn ammunition_recharge(
    _time: Res<Time>,
    _ammunition_maximum: Res<ProjectileAmmunitionMaximum>,
    _current_ammunition: ResMut<ProjectileAmmunition>,
) {

    // for (mine_entity, mut mine) in &mut mines {
    //     mine.mine.lifetime_weapon.lifetime.tick(time.delta());

    //     if mine.mine.lifetime_weapon.lifetime.finished() {
    //         commands.entity(mine_entity).despawn();

    //         tracing::info!("Mine despawn",);
    //     }
    // }
}
