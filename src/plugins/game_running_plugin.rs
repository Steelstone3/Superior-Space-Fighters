use crate::systems::{
    player::{player_movement::player_movement, player_weapon_select::player_weapon_select},
    weapons::{
        blaster::{
            blaster_lifetime::blaster_lifetime, blaster_movement::blaster_movement,
            spawn_blaster::spawn_blaster,
        },
        exotic::{
            exotic_lifetime::exotic_lifetime, exotic_movement::exotic_movement,
            spawn_exotic::spawn_exotic,
        },
        mine::{
            mine_lifetime::mine_lifetime, mine_movement::mine_movement, spawn_mine::spawn_mine,
        },
        torpedo::{
            spawn_torpedo::spawn_torpedo, torpedo_lifetime::torpedo_lifetime,
            torpedo_movement::torpedo_movement,
        },
    },
};
use bevy::prelude::{App, Plugin, Update};

pub struct GameRunningPlugin;

impl Plugin for GameRunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .add_systems(Update, player_weapon_select)
            .add_systems(Update, spawn_blaster)
            .add_systems(Update, spawn_torpedo)
            .add_systems(Update, spawn_mine)
            .add_systems(Update, spawn_exotic)
            .add_systems(Update, blaster_lifetime)
            .add_systems(Update, torpedo_lifetime)
            .add_systems(Update, mine_lifetime)
            .add_systems(Update, exotic_lifetime)
            .add_systems(Update, blaster_movement)
            .add_systems(Update, torpedo_movement)
            .add_systems(Update, mine_movement)
            .add_systems(Update, exotic_movement);
    }
}
