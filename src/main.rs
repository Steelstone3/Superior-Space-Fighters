use std::env;

use bevy::prelude::*;
use plugins::{
    ai::ai_plugin::AIPlugin,
    groups::{
        audio_plugin_group::AudioPluginGroup, core_plugin_group::CorePluginGroup,
        input_plugin_group::InputPluginGroup, player_plugin_group::PlayerPluginGroup,
        space_plugin_group::SpacePluginGroup, sprite_plugin_group::SpritePluginGroup,
    },
};

mod assets;
mod components;
mod events;
mod plugins;
mod queries;
mod resources;
mod systems;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
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
            CorePluginGroup,
            SpritePluginGroup,
            AudioPluginGroup,
            PlayerPluginGroup,
            InputPluginGroup,
            SpacePluginGroup,
            AIPlugin,
        ))
        .run();
}
