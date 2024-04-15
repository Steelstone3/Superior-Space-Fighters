use bevy::app::Plugin;

use super::{sprite_events::SpriteEvents, sprite_update_plugin::SpriteUpdatePlugin};

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((SpriteEvents, SpriteUpdatePlugin));
    }
}
