use bevy::ecs::event::Event;

#[derive(Event)]
pub struct UserInterfaceEvent;

#[derive(Event)]
pub struct PauseMenuEvent;

#[derive(Event)]
pub struct MainMenuEvent;
