use crate::{
    resources::greet_people::{greet_people, GreetTimer},
    systems::{add_people::add_people, camera::add_camera},
};
use bevy::{
    prelude::{App, Camera2dBundle, Commands, Plugin, Startup, Update},
    time::{Timer, TimerMode},
};

pub struct GameStartupPlugin;

impl Plugin for GameStartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
    }
}
