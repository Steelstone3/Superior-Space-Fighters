use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    resources::targetting_settings::TargettingSettingsResource,
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

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
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
                .run_if(run_if_not_paused),
        );

        app.insert_resource(TargettingSettingsResource {
            maximum_distance: 2000.0,
            is_targetting: false,
        });
    }
}
