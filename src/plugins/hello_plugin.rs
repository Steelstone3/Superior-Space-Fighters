use crate::{
    resources::greet_people::{greet_people, GreetTimer},
    systems::add_people::add_people,
};
use bevy::{
    prelude::{App, Plugin, Startup, Update},
    time::{Timer, TimerMode},
};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}
