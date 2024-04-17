use bevy::{app::Plugin, ecs::schedule::OnEnter};

use crate::{
    states::core_states::GameState,
    systems::audio::player_engine_rumble_sound::player_engine_rumble_sound,
};

pub struct PlayerSoundEffectsUpdatePlugin;

impl Plugin for PlayerSoundEffectsUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::InGame), player_engine_rumble_sound);
    }
}
