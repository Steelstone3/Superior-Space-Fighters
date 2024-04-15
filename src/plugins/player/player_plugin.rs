use bevy::app::Plugin;

use super::{player_start_plugin::PlayerStartPlugin, player_update_plugin::PlayerUpdatePlugin};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((PlayerUpdatePlugin, PlayerStartPlugin));
    }
}
