use bevy::prelude::*;
use systems::{add_people::add_people, greet_people::greet_people, hello_world::hello_world};

mod components;
mod systems;
mod entities;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
