use crate::systems::weapons::player_weapons::{
    player_blaster::fire_player_blaster::fire_player_blaster,
    player_exotic::fire_player_exotic::fire_player_exotic,
    player_mine::fire_player_mine::fire_player_mine,
    player_torpedo::fire_player_torpedo::fire_player_torpedo,
};
use bevy::app::{Plugin, Update};

pub struct CombatEventHandlers;

impl Plugin for CombatEventHandlers {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, fire_player_blaster)
            .add_systems(Update, fire_player_torpedo)
            .add_systems(Update, fire_player_mine)
            .add_systems(Update, fire_player_exotic);
    }
}
