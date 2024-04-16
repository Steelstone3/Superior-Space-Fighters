use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::weapons::player_weapons::{
        player_blaster::fire_player_blaster::fire_player_blaster,
        player_exotic::fire_player_exotic::fire_player_exotic,
        player_mine::fire_player_mine::fire_player_mine,
        player_torpedo::fire_player_torpedo::fire_player_torpedo,
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

pub struct CombatEventHandlersPlugin;

impl Plugin for CombatEventHandlersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, fire_player_blaster.run_if(run_if_not_paused))
            .add_systems(Update, fire_player_torpedo.run_if(run_if_not_paused))
            .add_systems(Update, fire_player_mine.run_if(run_if_not_paused))
            .add_systems(Update, fire_player_exotic.run_if(run_if_not_paused));
    }
}
