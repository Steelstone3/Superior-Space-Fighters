use crate::systems::{
    camera::spawn_player_camera::spawn_player_camera,
    music::exploration_music::play_exploration_music, planets::spawn_planet::spawn_random_planets,
    player::spawn_player::spawn_player_ship, ships::spawn_ships::spawn_random_ships,
    space::spawn_space::spawn_random_empty_space_background,
    stations::spawn_station::spawn_random_station,
    user_interface::weapons::weapon_selection::weapon_selection,
    weapons::targetting::spawn_targetting_settings::spawn_targetting_setting,
};
use bevy::{
    app::Update,
    prelude::{App, Plugin, Startup},
};

pub struct GameUserInterfacePlugin;

impl Plugin for GameUserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, weapon_selection);
    }
}
