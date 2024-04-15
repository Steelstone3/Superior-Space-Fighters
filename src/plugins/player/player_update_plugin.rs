use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::player::{
        player_movement::player_movement, player_pause_resume_game::player_pause_resume,
    },
};

pub struct PlayerUpdatePlugin;

impl Plugin for PlayerUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, player_movement.run_if(run_if_not_paused));
        app.add_systems(Update, player_pause_resume);
    }
}
