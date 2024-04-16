use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::event_handlers::logging_event_handlers::logging,
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

pub struct CoreEventHandlersPlugin;

impl Plugin for CoreEventHandlersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, logging.run_if(run_if_not_paused));
    }
}
