use bevy::app::Plugin;

use crate::plugins::events::{
    sprite_event_handlers::SpriteEventHandlers, sprite_events::SpriteEvents,
};

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((SpriteEvents, SpriteEventHandlers));
    }
}
