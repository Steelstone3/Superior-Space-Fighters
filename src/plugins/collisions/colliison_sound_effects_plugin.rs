use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    events::collision_events::{
        PlayerBlasterCollisionEvent, PlayerExoticCollisionEvent, PlayerMineCollisionEvent,
        PlayerTorpedoCollisionEvent,
    },
    plugins::run_conditions::event_called,
    states::core_states::GameState,
    systems::weapons::player_weapons::{
        player_blaster::spawn_player_blaster_collision_sound::spawn_player_blaster_collision_sound,
        player_exotic::spawn_player_exotic_collision_sound::spawn_player_exotic_collision_sound,
        player_mine::spawn_player_mine_collision_sound::spawn_player_mine_collision_sound,
        player_torpedo::spawn_player_torpedo_collision_sound::spawn_player_torpedo_collision_sound,
    },
};

pub struct CollisionSoundEffectsPlugin;

impl Plugin for CollisionSoundEffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            spawn_player_blaster_collision_sound
                .run_if(event_called::<PlayerBlasterCollisionEvent>),
        )
        .add_systems(
            Update,
            spawn_player_torpedo_collision_sound
                .run_if(event_called::<PlayerTorpedoCollisionEvent>),
        )
        .add_systems(
            Update,
            spawn_player_mine_collision_sound.run_if(event_called::<PlayerMineCollisionEvent>),
        )
        .add_systems(
            Update,
            spawn_player_exotic_collision_sound.run_if(event_called::<PlayerExoticCollisionEvent>),
        );
    }
}
