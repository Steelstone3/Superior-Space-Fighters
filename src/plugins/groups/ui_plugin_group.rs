use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::user_interface::{
    user_interface_events_plugin::UserInterfaceEventsPlugin,
    user_interface_menus_startup_plugin::UserInterfaceMenusStartupPlugin,
    user_interface_menus_update_plugin::UserInterfaceMenusUpdatePlugin,
    user_interface_targeting_plugin::UserInterfaceTargetingPlugin,
    user_interface_targeting_resources_plugin::UserInterfaceTargetingResourcesPlugin,
    user_interface_weapon_selection_update_plugin::UserInterfaceWeaponSelectionUpdatePlugin,
};

pub struct UserInterfacePluginGroup;

impl PluginGroup for UserInterfacePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UserInterfaceEventsPlugin)
            .add(UserInterfaceTargetingPlugin)
            .add(UserInterfaceWeaponSelectionUpdatePlugin)
            .add(UserInterfaceMenusUpdatePlugin)
            .add(UserInterfaceMenusStartupPlugin)
            .add(UserInterfaceTargetingResourcesPlugin)
    }
}
