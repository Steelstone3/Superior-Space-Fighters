use bevy::prelude::*;
use plugins::game_startup_plugin::GameStartupPlugin;

mod components;
mod entities;
mod plugins;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameStartupPlugin))
        .run();
}
