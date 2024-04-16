use bevy::app::Plugin;

use crate::events::user_interface_events::{PauseMenuEvent, UserInterfaceEvent};

pub struct UserInterfaceEventsPlugin;

impl Plugin for UserInterfaceEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UserInterfaceEvent>()
            .add_event::<PauseMenuEvent>();
    }
}
