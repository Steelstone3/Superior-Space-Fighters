use crate::{
    plugins::run_conditions::run_if_not_paused, systems::event_handlers::spawn_audio::spawn_audio,
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

pub struct AudioEventHandlers;

impl Plugin for AudioEventHandlers {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_audio.run_if(run_if_not_paused));
    }
}
