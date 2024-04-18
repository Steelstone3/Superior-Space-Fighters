use bevy::app::Plugin;

use crate::resources::projectile_ammunition_resource::ProjectileAmmunitionResource;

pub struct WeaponSaveTypesPlugin;

impl Plugin for WeaponSaveTypesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ProjectileAmmunitionResource>();
    }
}
