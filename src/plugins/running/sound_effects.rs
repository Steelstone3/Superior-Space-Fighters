use bevy::app::{Plugin, Startup, Update};

use crate::systems::{
    player::player_engine_rumble_sound::player_engine_rumble_sound,
    weapons::player_weapons::{
        player_blaster::{
            spawn_player_blaster_sound::spawn_player_blaster_sound,
            spawn_player_blaster_collision_sound::spawn_player_blaster_collision_sound,
        },
        player_exotic::{
            spawn_player_exotic_sound::spawn_player_exotic_sound,
            spawn_player_exotic_collision_sound::spawn_player_exoitc_collision_sound,
        },
        player_mine::{
            spawn_player_mine_audio::spawn_player_mine_sound,
            spawn_player_mine_collision_sound::spawn_player_mine_collision_sound,
        },
        player_torpedo::{
            spawn_player_torpedo_audio::spawn_player_torpedo_sound,
            spawn_player_torpedo_collision_sound::spawn_player_torpedo_collision_sound,
        },
    },
};

pub struct SoundEffects;

impl Plugin for SoundEffects {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, player_engine_rumble_sound)
            .add_systems(Update, spawn_player_blaster_sound)
            .add_systems(Update, spawn_player_blaster_collision_sound)
            .add_systems(Update, spawn_player_torpedo_sound)
            .add_systems(Update, spawn_player_torpedo_collision_sound)
            .add_systems(Update, spawn_player_mine_sound)
            .add_systems(Update, spawn_player_mine_collision_sound)
            .add_systems(Update, spawn_player_exotic_sound)
            .add_systems(Update, spawn_player_exoitc_collision_sound);
    }
}
