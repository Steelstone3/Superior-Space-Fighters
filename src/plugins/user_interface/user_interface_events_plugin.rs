use bevy::app::Plugin;

use crate::events::{
    game_state_events::PauseEvent, user_interface_events::InGameUserInterfaceEvent,
};

pub struct UserInterfaceEventsPlugin;

impl Plugin for UserInterfaceEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<InGameUserInterfaceEvent>();
        app.add_event::<PauseEvent>();
    }
}
