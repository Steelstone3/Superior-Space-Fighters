use bevy::ecs::event::Event;

#[derive(Event)]
pub struct LoggingEvent {
    pub message: String,
}
