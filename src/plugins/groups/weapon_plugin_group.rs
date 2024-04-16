use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::weapons::{
    weapon_blaster_plugin::WeaponBlasterPlugin,
    weapon_event_handlers_plugin::WeaponEventHandlersPlugin,
    weapon_events_plugin::WeaponEventsPlugin, weapon_exotic_plugin::WeaponExoticPlugin,
    weapon_mine_plugin::WeaponMinePlugin, weapon_resources_plugin::WeaponResourcesPlugin,
    weapon_selection_plugin::WeaponSelectionPlugin,
    weapon_sound_effects_plugin::WeaponSoundEffectsPlugin,
    weapon_torpedo_plugin::WeaponTorpedoPlugin, weapons_ammunition_plugin::WeaponsAmmunitionPlugin,
};

pub struct WeaponPluginGroup;

impl PluginGroup for WeaponPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WeaponEventsPlugin)
            .add(WeaponEventHandlersPlugin)
            .add(WeaponSoundEffectsPlugin)
            .add(WeaponsAmmunitionPlugin)
            .add(WeaponSelectionPlugin)
            .add(WeaponBlasterPlugin)
            .add(WeaponMinePlugin)
            .add(WeaponTorpedoPlugin)
            .add(WeaponExoticPlugin)
            .add(WeaponResourcesPlugin)
    }
}
