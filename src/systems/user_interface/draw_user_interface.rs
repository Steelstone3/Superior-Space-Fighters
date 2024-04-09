use bevy::ecs::event::EventWriter;

use crate::events::user_interface_events::UserInterfaceEvent;

pub fn draw_user_interface(mut draw_ui_event: EventWriter<UserInterfaceEvent>) {
    draw_ui_event.send(UserInterfaceEvent {});
}
