use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::{
        player::player_movement::player_movement,
        space::move_empty_space::move_empty_space,
        starships::starship_movement::ai_movement,
        weapons::player_weapons::{
            player_blaster::player_blaster_movement::player_blaster_movement,
            player_exotic::player_exotic_movement::player_exotic_movement,
            player_mine::player_mine_movement::player_mine_movement,
            player_torpedo::player_torpedo_movement::player_torpedo_movement,
        },
    },
};

pub struct Movement;

impl Plugin for Movement {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, move_empty_space.run_if(run_if_not_paused))
            .add_systems(Update, player_movement.run_if(run_if_not_paused))
            .add_systems(Update, ai_movement.run_if(run_if_not_paused))
            .add_systems(Update, player_blaster_movement.run_if(run_if_not_paused))
            .add_systems(Update, player_torpedo_movement.run_if(run_if_not_paused))
            .add_systems(Update, player_mine_movement.run_if(run_if_not_paused))
            .add_systems(Update, player_exotic_movement.run_if(run_if_not_paused));
    }
}
