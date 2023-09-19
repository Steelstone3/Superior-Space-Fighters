use bevy::prelude::*;
use plugins::hello_plugin::HelloPlugin;

mod components;
mod entities;
mod systems;
mod plugins;
mod resources;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}
