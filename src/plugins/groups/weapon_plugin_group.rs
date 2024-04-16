use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::weapons::{
    weapon_event_handlers_plugin::CombatEventHandlersPlugin,
    weapon_events_plugin::CombatEventsPlugin,
    weapon_sound_effects_plugin::WeaponSoundEffectsPlugin, weapons_plugin::WeaponsPlugin,
};

pub struct WeaponPluginGroup;

impl PluginGroup for WeaponPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WeaponsPlugin)
            .add(CombatEventsPlugin)
            .add(CombatEventHandlersPlugin)
            .add(WeaponSoundEffectsPlugin)
    }
}
