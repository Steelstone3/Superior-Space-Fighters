use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    states::core_states::GameState,
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

pub struct UserInterfaceTargetingPlugin;

impl Plugin for UserInterfaceTargetingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                spawn_combat_target,
                combat_target_movement,
                despawn_combat_target,
                spawn_trading_target,
                trading_target_movement,
                despawn_trading_target,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
