use crate::events::logging_event::LoggingEvent;
use bevy::app::Plugin;

pub struct CoreEventsPlugin;

impl Plugin for CoreEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<LoggingEvent>();
    }
}
