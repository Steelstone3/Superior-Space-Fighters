use super::{camera::PlayerCamera, music::Music, spawn_world::SpawnWorld, spawning::Spawning};
use bevy::prelude::{App, Plugin};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerCamera)
            .add_plugins(Music)
            .add_plugins(SpawnWorld)
            .add_plugins(Spawning);
    }
}
