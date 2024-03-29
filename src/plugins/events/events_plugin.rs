use bevy::app::Plugin;

use super::{
    combat_event_handlers::CombatEventHandlers, combat_events::CombatEvents,
    spawning_events::SpawningEvents, user_interface_events::UserInterfaceEvents,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(SpawningEvents)
            .add_plugins(UserInterfaceEvents)
            .add_plugins(CombatEvents)
            .add_plugins(CombatEventHandlers);
    }
}
