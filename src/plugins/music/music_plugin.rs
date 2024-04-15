use bevy::app::Plugin;

use super::{
    music_resources_plugin::MusicResourcesPlugin, music_startup_plugin::MusicStartupPlugin,
};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((MusicStartupPlugin, MusicResourcesPlugin));
    }
}
