use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::starships::{
        despawn_starships::despawn_destoryed_starships, spawn_starships, starship_movement,
    },
};

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_starships::spawn_random_starships)
            .add_systems(
                Update,
                starship_movement::ai_movement.run_if(run_if_not_paused),
            )
            .add_systems(Update, despawn_destoryed_starships);
    }
}
