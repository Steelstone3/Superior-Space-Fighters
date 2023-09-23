use bevy::prelude::*;
use plugins::game_startup_plugin::GameStartupPlugin;

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
            GameStartupPlugin,
        ))
        .run();
}
