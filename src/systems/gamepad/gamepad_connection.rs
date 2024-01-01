use bevy::{
    ecs::{
        event::EventReader,
        system::Res,
    },
    input::gamepad::{Gamepad, GamepadConnection, GamepadConnectionEvent},
    prelude::{Commands, Resource},
    utils::tracing,
};

#[derive(Resource)]
pub struct MyGamepad(Gamepad);

pub fn single_gamepad_connection(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_event_reader: EventReader<GamepadConnectionEvent>,
) {
    match my_gamepad {
        Some(_) => {
            for event in gamepad_event_reader.read() {
                match event.connection {
                    GamepadConnection::Connected(_) => {
                        commands.insert_resource(MyGamepad(event.gamepad));
                        tracing::info!("Controller connected");
                    }
                    GamepadConnection::Disconnected => {
                        commands.remove_resource::<MyGamepad>();
                        tracing::info!("Controller disconnected");
                    }
                }
            }
        }
        None => {}
    }
}
