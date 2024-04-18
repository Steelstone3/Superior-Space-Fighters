use bevy::app::{Plugin, Update};

use crate::systems::{
    save_load_game::save_load_game_state::save_load_game_state,
    starships::spawn_starship_sprite_on_load::spawn_starship_sprite_on_load,
};

pub struct SaveLoadGamePlugin;

impl Plugin for SaveLoadGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, save_load_game_state);
        app.add_systems(Update, spawn_starship_sprite_on_load);
    }
}
