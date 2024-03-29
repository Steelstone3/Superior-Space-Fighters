use super::{logging::Logging, targetting::Targetting, weapon_selection::WeaponSelection};
use bevy::app::Plugin;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(Targetting)
            .add_plugins(WeaponSelection)
            .add_plugins(Logging);
    }
}
