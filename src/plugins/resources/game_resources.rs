use bevy::prelude::{App, Plugin};

use super::{
    combat::Combat, player_camera::PlayerCamera, sound::Sound, trading::Trading, world::World,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(World)
            .add_plugins(Trading)
            .add_plugins(Combat)
            .add_plugins(PlayerCamera)
            .add_plugins(Sound);
    }
}
