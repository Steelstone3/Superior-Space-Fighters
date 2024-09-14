use crate::{
    states::core_states::GameState,
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
    prelude::in_state,
};

pub struct WeaponEventHandlersPlugin;

impl Plugin for WeaponEventHandlersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                fire_player_blaster,
                fire_player_torpedo,
                fire_player_mine,
                fire_player_exotic,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
