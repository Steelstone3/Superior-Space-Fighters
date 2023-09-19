use bevy::{
    prelude::{Query, Res, ResMut, Resource, With},
    time::{Time, Timer},
};

use crate::components::{name::Name, person::Person};

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

pub fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
