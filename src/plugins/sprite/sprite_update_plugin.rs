use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
    prelude::in_state,
};

use crate::{
    states::core_states::GameState,
    systems::event_handlers::sprite_event_handlers::{despawn_sprite, spawn_sprite},
};

pub struct SpriteUpdatePlugin;

impl Plugin for SpriteUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (spawn_sprite, despawn_sprite).run_if(in_state(GameState::InGame)),
        );
    }
}
