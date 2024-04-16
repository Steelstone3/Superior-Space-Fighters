use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::weapons::player_weapons::ammunition_recharge::ammunition_recharge,
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

pub struct WeaponsAmmunitionPlugin;

impl Plugin for WeaponsAmmunitionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (ammunition_recharge,).run_if(run_if_not_paused));
    }
}
