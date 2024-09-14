use bevy::app::Plugin;

use crate::events::user_interface_events::{
    InGameUserInterfaceEvent, LoadGameEvent, NewGameEvent, PauseEvent, SaveGameEvent,
};

pub struct UserInterfaceEventsPlugin;

impl Plugin for UserInterfaceEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<InGameUserInterfaceEvent>();
        app.add_event::<PauseEvent>();
        app.add_event::<NewGameEvent>();
        app.add_event::<SaveGameEvent>();
        app.add_event::<LoadGameEvent>();
    }
}
