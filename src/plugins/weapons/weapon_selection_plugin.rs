use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::player::player_weapon_select::player_weapon_select,
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

pub struct WeaponSelectionPlugin;

impl Plugin for WeaponSelectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, player_weapon_select.run_if(run_if_not_paused));
    }
}
