use bevy::app::{Plugin, Update};

use crate::systems::{
    trading::targetting::despawn_trading_target::despawn_trading_target,
    weapons::{
        player_blaster::player_blaster_lifetime::player_blaster_lifetime,
        player_exotic::player_exotic_lifetime::player_exotic_lifetime,
        player_mine::player_mine_lifetime::player_mine_lifetime,
        player_torpedo::player_torpedo_lifetime::player_torpedo_lifetime,
        targetting::despawn_combat_target::despawn_combat_target,
    },
};

pub struct Lifetime;

impl Plugin for Lifetime {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, despawn_trading_target)
            .add_systems(Update, despawn_combat_target)
            .add_systems(Update, player_blaster_lifetime)
            .add_systems(Update, player_torpedo_lifetime)
            .add_systems(Update, player_mine_lifetime)
            .add_systems(Update, player_exotic_lifetime);
    }
}
