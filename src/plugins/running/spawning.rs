use crate::systems::{
    player::player_weapon_select::player_weapon_select,
    trading::targetting::spawn_trading_target::spawn_trading_target,
    weapons::{
        player_blaster::spawn_player_blaster::spawn_player_blaster,
        player_exotic::spawn_player_exotic::spawn_player_exotic,
        player_mine::spawn_player_mine::spawn_player_mine,
        player_torpedo::spawn_player_torpedo::spawn_player_torpedo,
        targetting::spawn_combat_target::spawn_combat_target,
    },
};
use bevy::app::{Plugin, Update};

pub struct Spawning;

impl Plugin for Spawning {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_combat_target)
            .add_systems(Update, spawn_trading_target)
            .add_systems(Update, player_weapon_select)
            .add_systems(Update, spawn_player_blaster)
            .add_systems(Update, spawn_player_torpedo)
            .add_systems(Update, spawn_player_mine)
            .add_systems(Update, spawn_player_exotic);
    }
}
