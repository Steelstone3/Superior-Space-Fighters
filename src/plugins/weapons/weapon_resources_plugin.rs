use crate::resources::{
    projectile_ammunition_recharge::ProjectileAmmunitionRecharge,
    projectile_ammunition_resource::ProjectileAmmunitionResource,
    selected_weapon::{SelectedWeaponEnum, SelectedWeaponResource},
};
use bevy::{
    app::Plugin,
    time::{Timer, TimerMode},
};

pub struct WeaponResourcesPlugin;

impl Plugin for WeaponResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ProjectileAmmunitionResource {
            blaster_ammunition: 20,
            maximum_blaster_ammunition: 20,
            torpedo_ammunition: 5,
            maximum_torpedo_ammunition: 5,
            mine_ammunition: 7,
            maximum_mine_ammunition: 7,
            exotic_ammunition: 2,
            maximum_exotic_ammunition: 2,
        })
        .insert_resource(SelectedWeaponResource {
            selected_weapon: SelectedWeaponEnum::Blaster as u32,
        })
        .insert_resource(ProjectileAmmunitionRecharge {
            blaster_recovery_rate: Timer::from_seconds(5.0, TimerMode::Repeating),
            torpedo_recovery_rate: Timer::from_seconds(15.0, TimerMode::Repeating),
            mine_recovery_rate: Timer::from_seconds(5.0, TimerMode::Repeating),
            exotic_recovery_rate: Timer::from_seconds(15.0, TimerMode::Repeating),
        });
    }
}
