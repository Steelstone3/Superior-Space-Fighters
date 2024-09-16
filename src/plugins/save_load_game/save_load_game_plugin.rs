use bevy::{
    app::{Plugin, Update},
    prelude::IntoSystemConfigs,
};

use crate::{
    events::user_interface_events::{LoadGameEvent, SaveGameEvent},
    plugins::run_conditions::event_called,
    systems::{
        save_load_game::save_load_game_state::{load_game_state, save_game_state},
        starships::spawn_starship_sprite_on_load::spawn_starship_sprite_on_load,
    },
};

pub struct SaveLoadGamePlugin;

impl Plugin for SaveLoadGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            save_game_state.run_if(event_called::<SaveGameEvent>),
        );
        app.add_systems(
            Update,
            load_game_state.run_if(event_called::<LoadGameEvent>),
        );
        app.add_systems(Update, spawn_starship_sprite_on_load);
    }
}
