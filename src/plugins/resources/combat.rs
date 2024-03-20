use bevy::{
    app::Plugin,
    time::{Timer, TimerMode},
};

use crate::resources::{
    projectile_ammunition::ProjectileAmmunition,
    projectile_ammunition_maximum::ProjectileAmmunitionMaximum,
    projectile_ammunition_recharge::ProjectileAmmunitionRecharge,
    projectile_fire_rate::ProjectileFireRate,
    selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
};

pub struct Combat;

impl Plugin for Combat {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ProjectileAmmunition {
            blaster_ammunition: 20,
            torpedo_ammunition: 5,
            mine_ammunition: 7,
            exotic_ammunition: 2,
        })
        .insert_resource(ProjectileAmmunitionMaximum {
            maximum_blaster_ammunition: 20,
            maximum_torpedo_ammunition: 5,
            maximum_mine_ammunition: 7,
            maximum_exotic_ammunition: 2,
        })
        .insert_resource(ProjectileFireRate {
            //Timers need to start at 0 as they start imedietly causing all weapons to have to wait one cooldown before fireing
            blaster_fire_rate: Timer::from_seconds(0.0, TimerMode::Once),
            torpedo_fire_rate: Timer::from_seconds(0.0, TimerMode::Once),
            mine_fire_rate: Timer::from_seconds(0.0, TimerMode::Once),
            exotic_fire_rate: Timer::from_seconds(0.0, TimerMode::Once),
        })
        .insert_resource(ProjectileAmmunitionRecharge {
            blaster_recovery_rate: Timer::from_seconds(5.0, TimerMode::Repeating),
            torpedo_recovery_rate: Timer::from_seconds(15.0, TimerMode::Repeating),
            mine_recovery_rate: Timer::from_seconds(5.0, TimerMode::Repeating),
            exotic_recovery_rate: Timer::from_seconds(15.0, TimerMode::Repeating),
        })
        .insert_resource(SelectedWeapon {
            selected_weapon: SelectedWeaponEnum::Blaster as u32,
        });
    }
}
