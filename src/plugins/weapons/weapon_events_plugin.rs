use crate::events::combat_events::{
    FirePlayerBlasterEvent, FirePlayerExoticEvent, FirePlayerMineEvent, FirePlayerTorpedoEvent,
};
use bevy::app::Plugin;

pub struct CombatEventsPlugin;

impl Plugin for CombatEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<FirePlayerBlasterEvent>()
            .add_event::<FirePlayerTorpedoEvent>()
            .add_event::<FirePlayerMineEvent>()
            .add_event::<FirePlayerExoticEvent>();
    }
}
