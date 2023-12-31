use bevy::{
    input::gamepad::{GamepadConnectionEvent, Gamepad, GamepadConnection},
    prelude::{Commands, Resource}, ecs::{system::Res, event::EventReader}, utils::tracing};

#[derive(Resource)]
pub struct MyGamepad(Gamepad);

pub fn single_gamepad_connection(
    mut _commands: Commands,
    _my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_event_reader: EventReader<GamepadConnectionEvent>,
) {
    for event in gamepad_event_reader.read() {
        match event.connection {
            GamepadConnection::Connected(_) => {
                let gamepad = event.gamepad;
                tracing::info!("Controller connected!");
            }
            GamepadConnection::Disconnected => {
                tracing::info!("Controller disconnected");
            }
        }
    }
}
