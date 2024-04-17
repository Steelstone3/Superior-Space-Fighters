use bevy::app::Plugin;

use crate::events::user_interface_events::InGameUserInterfaceEvent;

pub struct UserInterfaceEventsPlugin;

impl Plugin for UserInterfaceEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<InGameUserInterfaceEvent>();
    }
}
