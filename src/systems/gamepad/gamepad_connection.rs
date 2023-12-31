// use bevy::{
//     input::gamepad::{GamepadConnectionEvent, Gamepad},
//     prelude::{Commands, Resource}, ecs::{system::Res, event::EventReader},
// };

// #[derive(Resource)]
// struct MyGamepad(Gamepad);

// pub fn single_gamepad_connection(
//     mut commands: Commands,
//     my_gamepad: Option<Res<MyGamepad>>,
//     mut gamepad_event_reader: EventReader<GamepadConnectionEvent>,
// ) {
//     for GamepadConnectionEvent(id, kind) in gamepad_event_reader {
//         let id = gamepad_event.gamepad;
        
//     }
// }












// impl GamepadRegistration for World {
//     fn register_gamepad(&mut self, gamepad: Gamepad) {
//         // Generate a synthetic event
//         let gamepad_event = GamepadEvent(gamepad, GamepadEventType::Connected);
//         let mut events_resource = self.resource_mut::<Events<GamepadEvent>>();
//         events_resource.send(gamepad_event);

//         // Ensure that the Gamepads resource exists to avoid pointless panics
//         if self.get_resource::<Gamepads>().is_none() {
//             self.init_resource::<Gamepads>();
//         }

//         // Manually run the gamepad_connection_system on the World to process the event just sent
//         let mut system = IntoSystem::into_system(gamepad_connection_system);
//         system.initialize(self);
//         system.run((), self);
//    }
// }
