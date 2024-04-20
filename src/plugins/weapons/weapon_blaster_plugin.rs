use crate::{
    states::core_states::GameState,
    systems::weapons::player_weapons::player_blaster::{
        player_blaster_ammunition_consumption::player_blaster_ammunition_consumption,
        player_blaster_lifetime::player_blaster_lifetime,
        player_blaster_movement::player_blaster_movement,
        spawn_player_blaster_sprite::spawn_player_blaster_sprite,
        spawn_player_blaster_sprite_on_load::spawn_player_blaster_sprite_on_load,
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
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
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(Update, spawn_player_blaster_sprite_on_load);
    }
}
