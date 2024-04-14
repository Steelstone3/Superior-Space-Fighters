use crate::plugins::events::logging_event_handlers::LoggingEventHandlers;

use super::{menus::Menus, targetting::Targetting, weapon_selection::WeaponSelection};
use bevy::app::Plugin;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(Targetting)
            .add_plugins(WeaponSelection)
            .add_plugins(LoggingEventHandlers)
            .add_plugins(Menus);
    }
}
