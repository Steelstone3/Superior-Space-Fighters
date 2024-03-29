use crate::systems::event_handlers::logging::logging;
use bevy::app::{Plugin, Update};

pub struct Logging;

impl Plugin for Logging {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, logging);
    }
}
