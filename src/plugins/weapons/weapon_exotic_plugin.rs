use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::weapons::player_weapons::player_exotic::{
        player_exotic_ammunition_consumption::player_exotic_ammunition_consumption,
        player_exotic_lifetime::player_exotic_lifetime,
        player_exotic_movement::player_exotic_movement,
        spawn_player_exotic_sprite::spawn_player_exotic_sprite,
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

pub struct WeaponExoticPlugin;

impl Plugin for WeaponExoticPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                spawn_player_exotic_sprite,
                player_exotic_ammunition_consumption,
                player_exotic_lifetime,
                player_exotic_movement,
            )
                .run_if(run_if_not_paused),
        );
    }
}
