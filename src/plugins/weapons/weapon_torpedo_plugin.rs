use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::weapons::player_weapons::player_torpedo::{
        player_torpedo_ammunition_consumption::player_torpedo_ammunition_consumption,
        player_torpedo_lifetime::player_torpedo_lifetime,
        player_torpedo_movement::player_torpedo_movement,
        spawn_player_torpedo_sprite::spawn_player_torpedo_sprite,
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

pub struct WeaponTorpedoPlugin;

impl Plugin for WeaponTorpedoPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                spawn_player_torpedo_sprite,
                player_torpedo_ammunition_consumption,
                player_torpedo_lifetime,
                player_torpedo_movement,
            )
                .run_if(run_if_not_paused),
        );
    }
}
