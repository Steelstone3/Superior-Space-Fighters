use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    states::core_states::GameState,
    systems::audio::player_engine_rumble_sound::player_engine_rumble_sound,
};

pub struct PlayerSoundEffectsUpdatePlugin;

impl Plugin for PlayerSoundEffectsUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            player_engine_rumble_sound.run_if(in_state(GameState::InGame)),
        );
    }
}
