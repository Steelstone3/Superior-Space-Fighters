use bevy::ecs::event::Event;

#[derive(Event)]
pub struct InGameUserInterfaceEvent;

#[derive(Event)]
pub struct PauseMenuEvent;

#[derive(Event)]
pub struct MainMenuEvent;
