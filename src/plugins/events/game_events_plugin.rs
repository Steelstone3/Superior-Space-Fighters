use crate::events::UserInterfaceEvent;
use bevy::app::Plugin;

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UserInterfaceEvent>();
    }
}
