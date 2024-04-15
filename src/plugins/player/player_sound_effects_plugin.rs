use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::audio::player_engine_rumble_sound::player_engine_rumble_sound,
};

pub struct PlayerSoundEffectsPlugin;

impl Plugin for PlayerSoundEffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Startup,
            player_engine_rumble_sound.run_if(run_if_not_paused),
        );
    }
}
