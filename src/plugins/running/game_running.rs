use bevy::prelude::{App, Plugin};

use super::{
    collision::Collision, lifetime::Lifetime, sound_effects::SoundEffects, spawning::Spawning,
};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SoundEffects)
            .add_plugins(Spawning)
            .add_plugins(Lifetime)
            .add_plugins(Collision);
    }
}
