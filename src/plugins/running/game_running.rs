use bevy::prelude::{App, Plugin};

use super::{
    collision::Collision, lifetime::Lifetime, movement::Movement, player_camera::PlayerCamera,
    sound_effects::SoundEffects, spawning::Spawning,
};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerCamera)
            .add_plugins(SoundEffects)
            .add_plugins(Movement)
            .add_plugins(Spawning)
            .add_plugins(Lifetime)
            .add_plugins(Collision);
    }
}
