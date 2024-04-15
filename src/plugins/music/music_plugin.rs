use bevy::app::Plugin;

use crate::plugins::resources::sound::MusicResourcePlugin;

use super::music_startup_plugin::MusicStartupPlugin;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((MusicStartupPlugin, MusicResourcePlugin));
    }
}
