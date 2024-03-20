use bevy::app::Plugin;

use crate::resources::{
    projectile_ammunition::ProjectileAmmunition, projectile_fire_rate::ProjectileFireRate,
    weapon_selection::WeaponSelection,
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
        .insert_resource(ProjectileFireRate {})
        .insert_resource(WeaponSelection { selected_weapon: 1 });
    }
}
