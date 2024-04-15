use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::user_interface::{
    menu_plugin::MenuPlugin, targeting_plugin::TargetingPlugin,
    user_interface_events_plugin::UserInterfaceEventsPlugin,
    weapon_selection_plugin::WeaponSelectionPlugin,
};

pub struct UserInterfacePluginGroup;

impl PluginGroup for UserInterfacePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UserInterfaceEventsPlugin)
            .add(TargetingPlugin)
            .add(WeaponSelectionPlugin)
            .add(MenuPlugin)
    }
}
