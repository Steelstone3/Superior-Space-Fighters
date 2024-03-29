use bevy::app::Plugin;

use crate::events::{
    spawn_sprite_event::SpawnSpriteEvent, user_interface_event::UserInterfaceEvent,
};

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UserInterfaceEvent>()
            .add_event::<SpawnSpriteEvent>();
    }
}
