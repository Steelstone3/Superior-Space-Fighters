use bevy::prelude::*;
use plugins::{game_running_plugin::GameRunningPlugin, game_start_plugin::GameStartPlugin};

mod components;
mod entities;
mod plugins;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Superior Space Fighters".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            GameStartPlugin,
            GameRunningPlugin,
        ))
        .run();
}
