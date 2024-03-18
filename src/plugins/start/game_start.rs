use super::{spawn_music::SpawnMusic, spawn_starships::SpawnStarships, spawn_world::SpawnWorld};
use bevy::prelude::{App, Plugin};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpawnMusic)
            .add_plugins(SpawnWorld)
            .add_plugins(SpawnStarships);
    }
}
