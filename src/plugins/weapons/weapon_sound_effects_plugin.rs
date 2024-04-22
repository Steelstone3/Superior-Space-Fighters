use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    events::combat_events::{
        FirePlayerBlasterEvent, FirePlayerExoticEvent, FirePlayerMineEvent, FirePlayerTorpedoEvent,
    },
    plugins::run_conditions::event_called,
    systems::weapons::player_weapons::{
        player_blaster::spawn_player_blaster_sound::spawn_player_blaster_sound,
        player_exotic::spawn_player_exotic_sound::spawn_player_exotic_sound,
        player_mine::spawn_player_mine_sound::spawn_player_mine_sound,
        player_torpedo::spawn_player_torpedo_sound::spawn_player_torpedo_sound,
    },
};

pub struct WeaponSoundEffectsPlugin;

impl Plugin for WeaponSoundEffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            spawn_player_blaster_sound.run_if(event_called::<FirePlayerBlasterEvent>),
        )
        .add_systems(
            Update,
            spawn_player_torpedo_sound.run_if(event_called::<FirePlayerTorpedoEvent>),
        )
        .add_systems(
            Update,
            spawn_player_mine_sound.run_if(event_called::<FirePlayerMineEvent>),
        )
        .add_systems(
            Update,
            spawn_player_exotic_sound.run_if(event_called::<FirePlayerExoticEvent>),
        );
    }
}
