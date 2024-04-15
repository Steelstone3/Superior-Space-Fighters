use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::weapons::{
    combat_event_handlers::CombatEventHandlers, combat_events::CombatEvents,
    weapons_plugin::WeaponsPlugin, weapons_sound_effects_plugin::WeaponSoundEffectsPlugin,
};

pub struct WeaponPluginGroup;

impl PluginGroup for WeaponPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WeaponsPlugin)
            .add(CombatEvents)
            .add(CombatEventHandlers)
            .add(WeaponSoundEffectsPlugin)
    }
}
