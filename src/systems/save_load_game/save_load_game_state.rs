use bevy::{
    ecs::world::World,
    input::{keyboard::KeyCode, ButtonInput},
    utils::tracing,
};
use bevy_save::WorldSaveableExt;

use crate::components::save_load_pipelines::save_load_pipeline::DebugSaveLoadPipeline;

pub fn save_load_game_state(world: &mut World) {
    let keys = world.resource::<ButtonInput<KeyCode>>();
    if let Some(save_location) = bevy_save::SAVE_DIR.to_str() {
        if keys.just_pressed(KeyCode::KeyU) {
            tracing::info!("Saving Game");
            match world.save(DebugSaveLoadPipeline(save_location)) {
                Ok(()) => (),
                Err(error) => tracing::info!("Failed to load save: {}", error),
            };
        }

        if keys.just_pressed(KeyCode::KeyJ) {
            tracing::info!("Loading Game");
            match world.load(DebugSaveLoadPipeline(save_location)) {
                Ok(()) => (),
                Err(error) => tracing::info!("Failed to load save: {}", error),
            };
        }
    }
}
