use crate::events::{logging_event::LoggingEvent, user_interface_event::UserInterfaceEvent};
use bevy::app::Plugin;

pub struct UserInterfaceEvents;

impl Plugin for UserInterfaceEvents {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UserInterfaceEvent>()
            .add_event::<LoggingEvent>();
    }
}
