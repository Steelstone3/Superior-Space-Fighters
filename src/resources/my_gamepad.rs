use bevy::{ecs::system::Resource, input::gamepad::Gamepad};

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);
