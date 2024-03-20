// use bevy::{ecs::world::World, utils::tracing};
// use bevy_save::WorldSaveableExt;

// use crate::components::save_load_pipelines::save_load_pipeline::SaveLoadPipeline;

// //replace SaveLoadPipeLine with DebugSaveLoadPipleline to debug save files
// pub fn save_game_state(world: &mut World) {
//     if let Some(save_location) = bevy_save::SAVE_DIR.to_str() {
//         match world.save(SaveLoadPipeline(save_location)) {
//             Ok(()) => (),
//             Err(error) => tracing::info!("Failed to load save: {}", error),
//         };
//     }
// }

// pub fn load_game_state(world: &mut World) {
//     if let Some(save_location) = bevy_save::SAVE_DIR.to_str() {
//         match world.load(SaveLoadPipeline(save_location)) {
//             Ok(()) => (),
//             Err(error) => tracing::info!("Failed to load save: {}", error),
//         };
//     }
// }
