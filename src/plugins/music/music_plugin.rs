use bevy::app::Plugin;

use super::{music_resources_plugin::MusicResourcesPlugin, music_update_plugin::MusicUpdatePlugin};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((MusicUpdatePlugin, MusicResourcesPlugin));
    }
}
