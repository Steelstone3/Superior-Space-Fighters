use crate::{
    states::core_states::GameState,
    systems::weapons::player_weapons::player_mine::{
        player_mine_ammunition_consumption::player_mine_ammunition_consumption,
        player_mine_lifetime::player_mine_lifetime, player_mine_movement::player_mine_movement,
        spawn_player_mine_sprite::spawn_player_mine_sprite,
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

pub struct WeaponMinePlugin;

impl Plugin for WeaponMinePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                spawn_player_mine_sprite,
                player_mine_ammunition_consumption,
                player_mine_lifetime,
                player_mine_movement,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
