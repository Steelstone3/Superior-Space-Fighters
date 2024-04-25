use bevy::{ecs::event::EventReader, utils::tracing};

use crate::events::logging_event::LoggingEvent;

pub fn logging(mut logging_events: EventReader<LoggingEvent>) {
    for logging_event in logging_events.read() {
        tracing::info!(logging_event.message);
    }
}
