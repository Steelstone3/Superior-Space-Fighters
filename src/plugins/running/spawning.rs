use crate::systems::{
    events::spawn_sprites::spawn_sprites, player::player_weapon_select::player_weapon_select, weapons::player_weapons::{
        player_blaster::spawn_player_blaster::spawn_player_blaster,
        player_exotic::spawn_player_exotic::spawn_player_exotic,
        player_mine::spawn_player_mine::spawn_player_mine,
        player_torpedo::spawn_player_torpedo::spawn_player_torpedo,
    }
};
use bevy::app::{Plugin, Update};

pub struct Spawning;

impl Plugin for Spawning {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, player_weapon_select)
            .add_systems(Update, spawn_player_blaster)
            .add_systems(Update, spawn_player_torpedo)
            .add_systems(Update, spawn_player_mine)
            .add_systems(Update, spawn_player_exotic)
            .add_systems(Update, spawn_sprites);
    }
}
