use bevy::app::{Plugin, Update};

use crate::systems::player::player_engine_rumble_sound::player_engine_rumble_sound;

pub struct SoundEffects;

impl Plugin for SoundEffects {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, player_engine_rumble_sound);
    }
}
