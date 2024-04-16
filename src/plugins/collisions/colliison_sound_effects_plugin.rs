use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::weapons::player_weapons::{
        player_blaster::spawn_player_blaster_collision_sound::spawn_player_blaster_collision_sound,
        player_exotic::spawn_player_exotic_collision_sound::spawn_player_exoitc_collision_sound,
        player_mine::spawn_player_mine_collision_sound::spawn_player_mine_collision_sound,
        player_torpedo::spawn_player_torpedo_collision_sound::spawn_player_torpedo_collision_sound,
    },
};

pub struct CollisionSoundEffectsPlugin;

impl Plugin for CollisionSoundEffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                spawn_player_blaster_collision_sound,
                spawn_player_torpedo_collision_sound,
                spawn_player_mine_collision_sound,
                spawn_player_exoitc_collision_sound,
            )
                .run_if(run_if_not_paused),
        );
    }
}
