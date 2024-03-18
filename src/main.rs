use std::env;

use bevy::prelude::*;
use plugins::{
    game_resources_plugin::GameResourcesPlugin, game_running_plugin::GameRunningPlugin,
    game_start_plugin::GameStartPlugin, game_user_interface_plugin::GameUserInterfacePlugin,
};

mod assets;
mod components;
mod plugins;
mod resources;
mod systems;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Superior Space Fighters".to_string(),
                        resolution: (640.0, 480.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            GameResourcesPlugin,
            GameStartPlugin,
            GameUserInterfacePlugin,
            GameRunningPlugin,
        ))
        .run();
}
