use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::weapons::player_weapons::player_blaster::{
        player_blaster_ammunition_consumption::player_blaster_ammunition_consumption,
        player_blaster_lifetime::player_blaster_lifetime,
        player_blaster_movement::player_blaster_movement,
        spawn_player_blaster_sprite::spawn_player_blaster_sprite,
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

pub struct WeaponBlasterPlugin;

impl Plugin for WeaponBlasterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                spawn_player_blaster_sprite,
                player_blaster_ammunition_consumption,
                player_blaster_lifetime,
                player_blaster_movement,
            )
                .run_if(run_if_not_paused),
        );
    }
}
