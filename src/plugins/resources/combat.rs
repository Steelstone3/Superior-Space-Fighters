use bevy::{
    app::Plugin,
    time::{Timer, TimerMode},
};

use crate::resources::{
    projectile_ammunition::ProjectileAmmunition,
    projectile_ammunition_maximum::ProjectileAmmunitionMaximum,
    projectile_fire_rate::ProjectileFireRate, weapon_selection::WeaponSelection,
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
            blaster_fire_rate: Timer::from_seconds(1.0, TimerMode::Once),
            torpedo_fire_rate: Timer::from_seconds(5.0, TimerMode::Once),
            mine_fire_rate: Timer::from_seconds(2.0, TimerMode::Once),
            exotic_fire_rate: Timer::from_seconds(10.0, TimerMode::Once),
        })
        .insert_resource(WeaponSelection { selected_weapon: 1 });
    }
}
