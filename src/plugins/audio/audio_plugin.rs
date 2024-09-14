use bevy::app::Plugin;

use super::{
    audio_event_handlers_plugin::AudioEventHandlersPlugin, audio_events_plugin::AudioEventsPlugin,
    audio_save_types_plugin::AudioSaveTypesPlugin,
};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            AudioEventHandlersPlugin,
            AudioEventsPlugin,
            AudioSaveTypesPlugin,
        ));
    }
}
