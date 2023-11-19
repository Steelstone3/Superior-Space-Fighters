use bevy::{
    asset::AssetServer,
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, Vec3},
    transform::components::Transform,
};

use crate::{
    components::player_starship::PlayerStarship,
    resources::chunks::{Chunk, Chunks},
    systems::space::spawn_space::spawn_random_space_background,
};

pub fn spawn_chunks(
    player_tran_query: Query<&Transform, With<PlayerStarship>>,
    mut chunks: ResMut<Chunks>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //TODO replace hardcoded numbers with values from ChunkSettingsResource

    //use player location to determine what chuncks to load
    let player_tran = player_tran_query.single().translation;
    let mut x_start = 0;
    let mut y_start = 0;

    if player_tran.x > 0. {
        x_start = (player_tran.x / 1920.).ceil().clamp(-4093., 4093.) as i32;
    } else if player_tran.x < 0. {
        x_start = (player_tran.x / 1920.).floor().clamp(-4093., 4093.) as i32;
    }

    if player_tran.y > 0. {
        y_start = (player_tran.y / 1920.).ceil().clamp(-4093., 4093.) as i32;
    } else if player_tran.y < 0. {
        y_start = (player_tran.y / 1920.).floor().clamp(-4093., 4093.) as i32;
    }

    print!("xstart: {}, ystart: {}", x_start, y_start);

    for x in x_start..x_start + 3 {
        for y in y_start..y_start + 3 {
            let x = x as f32;
            let y = y as f32;

            let chunk_id = Vec2::new(x, y);
            let spawn_location = Vec2::new((x * 1920.) - 2880. + 960., (y * 1920.) - 2880. + 960.);
            if !chunks.chunk_exists(chunk_id) {
                spawn_random_space_background(
                    &mut commands,
                    &asset_server,
                    Vec3::new(spawn_location.x, spawn_location.y, 0.),
                );
                chunks.chunks.push(Chunk {
                    chunk_id: chunk_id,
                    chunk_location: spawn_location,
                });
            }
        }
    }
}
