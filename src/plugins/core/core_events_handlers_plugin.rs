use crate::systems::{
    core::start_new_game::start_new_game, event_handlers::logging_event_handlers::logging,
};
use bevy::app::{Plugin, Update};

pub struct CoreEventHandlersPlugin;

impl Plugin for CoreEventHandlersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, logging)
            .add_systems(Update, start_new_game);
    }
}
