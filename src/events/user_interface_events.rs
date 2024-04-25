use bevy::ecs::event::Event;

#[derive(Event)]
pub struct InGameUserInterfaceEvent;

#[derive(Event)]
pub struct NewGameEvent;

#[derive(Event)]
pub struct LoadGameEvent;

#[derive(Event)]
pub struct SaveGameEvent;

#[derive(Event)]
pub struct PauseEvent {
    pub pause: bool,
}
