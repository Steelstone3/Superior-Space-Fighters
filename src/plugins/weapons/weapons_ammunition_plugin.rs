use crate::{
    states::core_states::GameState,
    systems::weapons::player_weapons::ammunition_recharge::ammunition_recharge,
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

pub struct WeaponsAmmunitionPlugin;

impl Plugin for WeaponsAmmunitionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            ammunition_recharge.run_if(in_state(GameState::InGame)),
        );
    }
}
