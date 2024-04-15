use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::{
        audio::player_engine_rumble_sound::player_engine_rumble_sound,
        weapons::player_weapons::{
            player_blaster::{
                spawn_player_blaster_collision_sound::spawn_player_blaster_collision_sound,
                spawn_player_blaster_sound::spawn_player_blaster_sound,
            },
            player_exotic::{
                spawn_player_exotic_collision_sound::spawn_player_exoitc_collision_sound,
                spawn_player_exotic_sound::spawn_player_exotic_sound,
            },
            player_mine::{
                spawn_player_mine_collision_sound::spawn_player_mine_collision_sound,
                spawn_player_mine_sound::spawn_player_mine_sound,
            },
            player_torpedo::{
                spawn_player_torpedo_collision_sound::spawn_player_torpedo_collision_sound,
                spawn_player_torpedo_sound::spawn_player_torpedo_sound,
            },
        },
    },
};

pub struct SoundEffectsPlugin;

impl Plugin for SoundEffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Startup,
            player_engine_rumble_sound.run_if(run_if_not_paused),
        )
        .add_systems(Update, spawn_player_blaster_sound.run_if(run_if_not_paused))
        .add_systems(
            Update,
            spawn_player_blaster_collision_sound.run_if(run_if_not_paused),
        )
        .add_systems(Update, spawn_player_torpedo_sound.run_if(run_if_not_paused))
        .add_systems(
            Update,
            spawn_player_torpedo_collision_sound.run_if(run_if_not_paused),
        )
        .add_systems(Update, spawn_player_mine_sound.run_if(run_if_not_paused))
        .add_systems(
            Update,
            spawn_player_mine_collision_sound.run_if(run_if_not_paused),
        )
        .add_systems(Update, spawn_player_exotic_sound.run_if(run_if_not_paused))
        .add_systems(
            Update,
            spawn_player_exoitc_collision_sound.run_if(run_if_not_paused),
        );
    }
}
