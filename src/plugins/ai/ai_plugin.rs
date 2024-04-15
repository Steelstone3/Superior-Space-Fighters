use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::starships::{spawn_starships, starship_movement},
};

pub struct AIPluginGroup;

impl Plugin for AIPluginGroup {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_starships::spawn_random_starships)
            .add_systems(
                Update,
                starship_movement::ai_movement.run_if(run_if_not_paused),
            );
    }
}
