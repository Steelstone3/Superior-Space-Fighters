use bevy::app::{Plugin, Startup};

use crate::systems::user_interface::weapon_selection::spawn_weapon_selection_icons;

pub struct WeaponSelection;

impl Plugin for WeaponSelection {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_weapon_selection_icons);
    }
}
