use crate::systems::user_interface::weapons::weapon_selection::weapon_selection;
use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

pub struct GameUserInterfacePlugin;

impl Plugin for GameUserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, weapon_selection);
    }
}
