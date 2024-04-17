use bevy::ecs::event::EventWriter;

use crate::events::user_interface_events::InGameUserInterfaceEvent;

pub fn initialize_ingame_interface(
    mut load_ingame_interface_event_writer: EventWriter<InGameUserInterfaceEvent>,
) {
    load_ingame_interface_event_writer.send(InGameUserInterfaceEvent {});
}
