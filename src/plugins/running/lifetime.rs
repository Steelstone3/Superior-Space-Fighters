use bevy::app::{Plugin, Update};

use crate::systems::weapons::player_weapons::{
    ammunition_recharge::ammunition_recharge,
    player_blaster::player_blaster_lifetime::player_blaster_lifetime,
    player_exotic::player_exotic_lifetime::player_exotic_lifetime,
    player_mine::player_mine_lifetime::player_mine_lifetime,
    player_torpedo::player_torpedo_lifetime::player_torpedo_lifetime,
};

pub struct Lifetime;

impl Plugin for Lifetime {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, player_blaster_lifetime)
            .add_systems(Update, player_torpedo_lifetime)
            .add_systems(Update, player_mine_lifetime)
            .add_systems(Update, player_exotic_lifetime)
            .add_systems(Update, ammunition_recharge);
    }
}
