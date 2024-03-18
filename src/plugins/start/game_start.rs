use super::{camera::PlayerCamera, sound::Sound, spawn_world::SpawnWorld, spawning::Spawning};
use bevy::prelude::{App, Plugin};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerCamera)
            .add_plugins(Sound)
            .add_plugins(SpawnWorld)
            .add_plugins(Spawning);
    }
}
