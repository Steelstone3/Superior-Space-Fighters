use bevy::ecs::event::{EventReader, EventWriter};

use crate::events::{game_state_events::NewGameEvent, user_interface_events::UserInterfaceEvent};

pub fn draw_user_interface(
    mut draw_ui_event: EventWriter<UserInterfaceEvent>,
    mut new_game_event_reader: EventReader<NewGameEvent>,
) {
    for _ in new_game_event_reader.read() {
        draw_ui_event.send(UserInterfaceEvent {});
    }
}
