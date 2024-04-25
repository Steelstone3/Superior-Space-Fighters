use crate::{
    events::combat_events::FirePlayerTorpedoEvent,
    plugins::run_conditions::event_called,
    states::core_states::GameState,
    systems::weapons::player_weapons::player_torpedo::{
        player_torpedo_ammunition_consumption::player_torpedo_ammunition_consumption,
        player_torpedo_lifetime::player_torpedo_lifetime,
        player_torpedo_movement::player_torpedo_movement,
        spawn_player_torpedo_sprite::spawn_player_torpedo_sprite,
        spawn_player_torpedo_sprite_on_load::spawn_player_torpedo_sprite_on_load,
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

pub struct WeaponTorpedoPlugin;

impl Plugin for WeaponTorpedoPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                spawn_player_torpedo_sprite,
                player_torpedo_ammunition_consumption,
            )
                .run_if(event_called::<FirePlayerTorpedoEvent>),
        )
        .add_systems(
            Update,
            (
                spawn_player_torpedo_sprite_on_load,
                player_torpedo_lifetime,
                player_torpedo_movement,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
