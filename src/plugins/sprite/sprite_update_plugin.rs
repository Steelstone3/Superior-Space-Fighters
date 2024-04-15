use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::event_handlers::{
        sprite_event_handlers::despawn_sprite, sprite_event_handlers::spawn_sprite,
    },
};

pub struct SpriteUpdatePlugin;

impl Plugin for SpriteUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (spawn_sprite, despawn_sprite).run_if(run_if_not_paused),
        );
    }
}
