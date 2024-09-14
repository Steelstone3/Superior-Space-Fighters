use std::env;

use bevy::prelude::*;
use plugins::groups::{
    ai_plugin_group::AIPluginGroup, collisions_plugin_group::CollisionPluginGroup,
    core_plugin_group::CorePluginGroup, music_plugin_group::MusicPluginGroup,
    player_plugin_group::PlayerPluginGroup, space_plugin_group::SpacePluginGroup,
    trading_plugin_group::TradingPluginGroup, ui_plugin_group::UserInterfacePluginGroup,
    weapon_plugin_group::WeaponPluginGroup,
};

mod assets;
mod components;
mod events;
mod plugins;
mod queries;
mod resources;
mod states;
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
            // SavePlugins,
            CorePluginGroup,
            PlayerPluginGroup,
            SpacePluginGroup,
            AIPluginGroup,
            UserInterfacePluginGroup,
            WeaponPluginGroup,
            CollisionPluginGroup,
            MusicPluginGroup,
            TradingPluginGroup,
            // SaveLoadPluginGroup,
        ))
        .run();
}
