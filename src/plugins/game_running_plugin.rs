use crate::systems::{
    camera::increment_camera::increment_camera,
    camera::{camera_movement::camera_movement, scroll_camera::scroll_camera},
    chunks::{despawn_chunks::despawn_chunks, spawn_chunks::spawn_chunks},
    player::{player_movement::player_movement, player_weapon_select::player_weapon_select},
    ships::ship_movement::ai_movement,
    weapons::{
        player_blaster::{
            player_blaster_lifetime::player_blaster_lifetime,
            player_blaster_movement::player_blaster_movement,
            spawn_player_blaster::spawn_player_blaster,
        },
        player_exotic::{
            player_exotic_lifetime::player_exotic_lifetime,
            player_exotic_movement::player_exotic_movement,
            spawn_player_exotic::spawn_player_exotic,
        },
        player_mine::{
            player_mine_lifetime::player_mine_lifetime, player_mine_movement::player_mine_movement,
            spawn_player_mine::spawn_player_mine,
        },
        player_torpedo::{
            player_torpedo_lifetime::player_torpedo_lifetime,
            player_torpedo_movement::player_torpedo_movement,
            spawn_player_torpedo::spawn_player_torpedo,
        },
    },
};
use bevy::prelude::{App, Plugin, Update};

pub struct GameRunningPlugin;

impl Plugin for GameRunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ai_movement)
            .add_systems(Update, camera_movement)
            .add_systems(Update, scroll_camera)
            .add_systems(Update, increment_camera)
            .add_systems(Update, player_movement)
            .add_systems(Update, player_weapon_select)
            .add_systems(Update, spawn_player_blaster)
            .add_systems(Update, spawn_player_torpedo)
            .add_systems(Update, spawn_player_mine)
            .add_systems(Update, spawn_player_exotic)
            .add_systems(Update, player_blaster_lifetime)
            .add_systems(Update, player_torpedo_lifetime)
            .add_systems(Update, player_mine_lifetime)
            .add_systems(Update, player_exotic_lifetime)
            .add_systems(Update, player_blaster_movement)
            .add_systems(Update, player_torpedo_movement)
            .add_systems(Update, player_mine_movement)
            .add_systems(Update, player_exotic_movement)
            .add_systems(Update, spawn_chunks)
            .add_systems(Update, despawn_chunks);
    }
}
