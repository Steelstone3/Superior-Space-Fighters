use bevy::{
    ecs::{
        event::EventReader,
        system::{Res, ResMut},
    },
    input::gamepad::{Gamepad, GamepadConnection, GamepadConnectionEvent},
    prelude::{Commands, Resource},
    utils::tracing,
};

#[derive(Resource)]
pub struct MyGamepad(Gamepad);

pub fn single_gamepad_connection(
    mut _commands: Commands,
    mut my_gamepad: Option<ResMut<MyGamepad>>,
    mut gamepad_event_reader: EventReader<GamepadConnectionEvent>,
) {
    match my_gamepad {
        Some(mut gamepad) => {
            for event in gamepad_event_reader.read() {
                match event.connection {
                    GamepadConnection::Connected(_) => {
                        gamepad.0 = event.gamepad;
                        tracing::info!("Controller connected");
                    }
                    GamepadConnection::Disconnected => {
                        tracing::info!("Controller disconnected");
                    }
                }
            }
        }
        None => {}
    }
}
