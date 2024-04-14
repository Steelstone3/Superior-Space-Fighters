use crate::events::logging_event::LoggingEvent;
use bevy::app::Plugin;

pub struct CoreEvents;

impl Plugin for CoreEvents {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<LoggingEvent>();
    }
}
