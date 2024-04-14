use super::{music::MusicPlugin, spawn_world::SpawnWorld, spawning::Spawning};
use bevy::prelude::{App, Plugin};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MusicPlugin)
            .add_plugins(SpawnWorld)
            .add_plugins(Spawning);
    }
}
