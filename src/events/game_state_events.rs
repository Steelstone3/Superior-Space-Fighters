use bevy::ecs::event::Event;

#[derive(Event)]
pub struct PauseEvent {
    pub pause: bool,
}

#[derive(Event)]
pub struct NewGameEvent;
