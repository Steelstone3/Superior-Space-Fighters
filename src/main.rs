use std::env;

use bevy::prelude::*;
use plugins::{
    events::game_events_plugin::GameEventsPlugin, resources::game_resources::ResourcesPlugin,
    running::game_running::RunningPlugin, start::game_start::StartPlugin,
    user_interface::game_user_interface::UserInterfacePlugin,
};

mod assets;
mod components;
mod events;
mod plugins;
mod queries;
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
            ResourcesPlugin,
            UserInterfacePlugin,
            StartPlugin,
            RunningPlugin,
            GameEventsPlugin,
        ))
        .run();
}
