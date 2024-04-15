use bevy::app::{Plugin, Update};

use crate::systems::user_interface::pause_menu::pause_resume_game;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, pause_resume_game);
    }
}
