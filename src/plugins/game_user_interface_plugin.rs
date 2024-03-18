use crate::systems::user_interface::weapons::{weapon_selection::weapon_selection, spawn_weapon_selection_icons::spawn_weapon_selection_icons};
use bevy::{
    app::{Startup, Update},
    prelude::{App, Plugin},
};

pub struct GameUserInterfacePlugin;

impl Plugin for GameUserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, weapon_selection);
        app.add_systems(Startup, spawn_weapon_selection_icons);
    }
}
