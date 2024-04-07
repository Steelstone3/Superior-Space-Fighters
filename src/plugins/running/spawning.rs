use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::{
        event_handlers::{
            despawn_sprite::despawn_sprite,
            spawn_audio::{pause_audio, play_audio, spawn_audio},
            spawn_sprite::spawn_sprite,
        },
        player::{
            player_pause_resume_game::player_pause_resume,
            player_weapon_select::player_weapon_select,
        },
        weapons::player_weapons::{
            player_blaster::{
                player_blaster_ammunition_consumption::player_blaster_ammunition_consumption,
                spawn_player_blaster_sprite::spawn_player_blaster_sprite,
            },
            player_exotic::{
                player_exotic_ammunition_consumption::player_exotic_ammunition_consumption,
                spawn_player_exotic_sprite::spawn_player_exotic_sprite,
            },
            player_mine::{
                player_mine_ammunition_consumption::player_mine_ammunition_consumption,
                spawn_player_mine_sprite::spawn_player_mine_sprite,
            },
            player_torpedo::{
                player_torpedo_ammunition_consumption::player_torpedo_ammunition_consumption,
                spawn_player_torpedo_sprite::spawn_player_torpedo_sprite,
            },
        },
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

pub struct Spawning;

impl Plugin for Spawning {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, player_weapon_select.run_if(run_if_not_paused))
            .add_systems(
                Update,
                spawn_player_blaster_sprite.run_if(run_if_not_paused),
            )
            .add_systems(
                Update,
                player_blaster_ammunition_consumption.run_if(run_if_not_paused),
            )
            .add_systems(
                Update,
                spawn_player_torpedo_sprite.run_if(run_if_not_paused),
            )
            .add_systems(
                Update,
                player_torpedo_ammunition_consumption.run_if(run_if_not_paused),
            )
            .add_systems(Update, spawn_player_mine_sprite.run_if(run_if_not_paused))
            .add_systems(
                Update,
                player_mine_ammunition_consumption.run_if(run_if_not_paused),
            )
            .add_systems(Update, spawn_player_exotic_sprite.run_if(run_if_not_paused))
            .add_systems(
                Update,
                player_exotic_ammunition_consumption.run_if(run_if_not_paused),
            )
            .add_systems(Update, spawn_sprite.run_if(run_if_not_paused))
            .add_systems(Update, despawn_sprite.run_if(run_if_not_paused))
            .add_systems(Update, spawn_audio.run_if(run_if_not_paused))
            .add_systems(Update, pause_audio)
            .add_systems(Update, play_audio)
            .add_systems(Update, player_pause_resume);
    }
}
