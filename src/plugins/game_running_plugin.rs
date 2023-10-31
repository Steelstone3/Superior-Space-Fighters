use crate::systems::{
    player::{player_movement::player_movement, player_weapon_select::player_weapon_select},
    weapons::{
        blaster_lifetime::blaster_lifetime, blaster_movement::blaster_movement,
        exotic_lifetime::exotic_lifetime, mine_lifetime::mine_lifetime,
        spawn_blaster::spawn_blaster, spawn_exotic::spawn_exotic, spawn_mine::spawn_mine,
        spawn_torpedo::spawn_torpedo, torpedo_lifetime::torpedo_lifetime,
    },
};
use bevy::prelude::{App, Plugin, Update};

pub struct GameRunningPlugin;

impl Plugin for GameRunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .add_systems(Update, player_weapon_select)
            .add_systems(Update, spawn_mine)
            .add_systems(Update, mine_lifetime)
            .add_systems(Update, spawn_exotic)
            .add_systems(Update, exotic_lifetime)
            .add_systems(Update, spawn_torpedo)
            .add_systems(Update, torpedo_lifetime)
            .add_systems(Update, spawn_blaster)
            .add_systems(Update, blaster_movement)
            .add_systems(Update, blaster_lifetime);
    }
}
