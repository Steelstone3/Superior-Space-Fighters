use bevy::prelude::{Query, With};

use crate::components::{name::Name, person::Person};

pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}