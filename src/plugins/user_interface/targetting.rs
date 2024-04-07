use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::user_interface::targetting::{
        combat::{
            combat_target_movement::combat_target_movement,
            despawn_combat_target::despawn_combat_target, spawn_combat_target::spawn_combat_target,
        },
        trading::{
            despawn_trading_target::despawn_trading_target,
            spawn_trading_target::spawn_trading_target,
            trading_target_movement::trading_target_movement,
        },
    },
};

pub struct Targetting;

impl Plugin for Targetting {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_combat_target.run_if(run_if_not_paused))
            .add_systems(Update, combat_target_movement.run_if(run_if_not_paused))
            .add_systems(Update, despawn_combat_target.run_if(run_if_not_paused))
            .add_systems(Update, spawn_trading_target.run_if(run_if_not_paused))
            .add_systems(Update, trading_target_movement.run_if(run_if_not_paused))
            .add_systems(Update, despawn_trading_target.run_if(run_if_not_paused));
    }
}
