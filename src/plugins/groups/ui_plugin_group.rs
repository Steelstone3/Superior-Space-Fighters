use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::user_interface::{
    user_interface_events_plugin::UserInterfaceEventsPlugin,
    user_interface_menus_plugin::UserInterfaceMenusPlugin,
    user_interface_targeting_plugin::UserInterfaceTargetingPlugin,
    user_interface_weapon_selection_plugin::UserInterfaceWeaponSelectionPlugin,
};

pub struct UserInterfacePluginGroup;

impl PluginGroup for UserInterfacePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UserInterfaceEventsPlugin)
            .add(UserInterfaceTargetingPlugin)
            .add(UserInterfaceWeaponSelectionPlugin)
            .add(UserInterfaceMenusPlugin)
    }
}
