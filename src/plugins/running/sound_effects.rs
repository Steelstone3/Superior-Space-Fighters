use bevy::app::{Plugin, Startup, Update};

use crate::systems::{
    player::player_engine_rumble_sound::player_engine_rumble_sound,
    weapons::player_weapons::{
        player_blaster::spawn_player_blaster_audio::spawn_player_blaster_audio,
        player_exotic::spawn_player_exotic_audio::spawn_player_exotic_audio,
        player_mine::spawn_player_mine_audio::spawn_player_mine_audio,
        player_torpedo::spawn_player_torpedo_audio::spawn_player_torpedo_audio,
    },
};

pub struct SoundEffects;

impl Plugin for SoundEffects {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, player_engine_rumble_sound)
            .add_systems(Update, spawn_player_blaster_audio)
            .add_systems(Update, spawn_player_torpedo_audio)
            .add_systems(Update, spawn_player_mine_audio)
            .add_systems(Update, spawn_player_exotic_audio);
    }
}
