use bevy::app::Plugin;

use crate::events::user_interface_events::{
    InGameUserInterfaceEvent, MainMenuEvent, PauseMenuEvent,
};

pub struct UserInterfaceEventsPlugin;

impl Plugin for UserInterfaceEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<InGameUserInterfaceEvent>()
            .add_event::<PauseMenuEvent>()
            .add_event::<MainMenuEvent>();
    }
}
