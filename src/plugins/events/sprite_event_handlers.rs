use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused, systems::event_handlers::spawn_sprite::spawn_sprite,
};

pub struct SpriteEventHandlers;

impl Plugin for SpriteEventHandlers {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_sprite.run_if(run_if_not_paused));
    }
}
