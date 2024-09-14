use crate::{
    events::combat_events::FirePlayerExoticEvent,
    plugins::run_conditions::event_called,
    states::core_states::GameState,
    systems::weapons::player_weapons::player_exotic::{
        player_exotic_ammunition_consumption::player_exotic_ammunition_consumption,
        player_exotic_lifetime::player_exotic_lifetime,
        player_exotic_movement::player_exotic_movement,
        spawn_player_exotic_sprite::spawn_player_exotic_sprite,
        spawn_player_exotic_sprite_on_load::spawn_player_exotic_sprite_on_load,
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

pub struct WeaponExoticPlugin;

impl Plugin for WeaponExoticPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                spawn_player_exotic_sprite,
                player_exotic_ammunition_consumption,
            )
                .run_if(event_called::<FirePlayerExoticEvent>),
        )
        .add_systems(
            Update,
            (
                player_exotic_movement,
                player_exotic_lifetime,
                spawn_player_exotic_sprite_on_load,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
