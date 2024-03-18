use crate::systems::{
    camera::{
        camera_movement::camera_movement, increment_camera::increment_camera,
        scroll_camera::scroll_camera,
    },
    player::{
        player_engine_rumble_sound::player_engine_rumble_sound, player_movement::player_movement,
        player_weapon_select::player_weapon_select,
    },
    ships::ship_movement::ai_movement,
    space::move_empty_space::move_empty_space,
    trading::targetting::{
        despawn_trading_target::despawn_trading_target, spawn_trading_target::spawn_trading_target,
        trading_target_movement::trading_target_movement,
    },
    weapons::{
        player_blaster::{
            player_blaster_collision::player_blaster_collision_with_starship,
            player_blaster_lifetime::player_blaster_lifetime,
            player_blaster_movement::player_blaster_movement,
            spawn_player_blaster::spawn_player_blaster,
        },
        player_exotic::{
            player_exotic_collision::player_exotic_collision_with_starship,
            player_exotic_lifetime::player_exotic_lifetime,
            player_exotic_movement::player_exotic_movement,
            spawn_player_exotic::spawn_player_exotic,
        },
        player_mine::{
            player_mine_collision::player_mine_collision_with_starship,
            player_mine_lifetime::player_mine_lifetime, player_mine_movement::player_mine_movement,
            spawn_player_mine::spawn_player_mine,
        },
        player_torpedo::{
            player_torpedo_collision::player_torpedo_collision_with_starship,
            player_torpedo_lifetime::player_torpedo_lifetime,
            player_torpedo_movement::player_torpedo_movement,
            spawn_player_torpedo::spawn_player_torpedo,
        },
        targetting::{
            combat_target_movement::combat_target_movement,
            despawn_combat_target::despawn_combat_target, spawn_combat_target::spawn_combat_target,
        },
    },
};
use bevy::prelude::{App, Plugin, Update};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ai_movement)
            .add_systems(Update, camera_movement)
            .add_systems(Update, scroll_camera)
            .add_systems(Update, increment_camera)
            .add_systems(Update, move_empty_space)
            .add_systems(Update, spawn_trading_target)
            .add_systems(Update, despawn_trading_target)
            .add_systems(Update, trading_target_movement)
            .add_systems(Update, player_movement)
            .add_systems(Update, player_engine_rumble_sound)
            .add_systems(Update, player_weapon_select)
            .add_systems(Update, spawn_player_blaster)
            .add_systems(Update, player_blaster_movement)
            .add_systems(Update, player_blaster_lifetime)
            .add_systems(Update, player_blaster_collision_with_starship)
            .add_systems(Update, spawn_player_torpedo)
            .add_systems(Update, player_torpedo_movement)
            .add_systems(Update, player_torpedo_lifetime)
            .add_systems(Update, player_torpedo_collision_with_starship)
            .add_systems(Update, spawn_combat_target)
            .add_systems(Update, despawn_combat_target)
            .add_systems(Update, combat_target_movement)
            .add_systems(Update, spawn_player_mine)
            .add_systems(Update, player_mine_movement)
            .add_systems(Update, player_mine_lifetime)
            .add_systems(Update, player_mine_collision_with_starship)
            .add_systems(Update, spawn_player_exotic)
            .add_systems(Update, player_exotic_movement)
            .add_systems(Update, player_exotic_lifetime)
            .add_systems(Update, player_exotic_collision_with_starship);
    }
}