use bevy::{ecs::system::{ResMut, Commands, Res}, math::{Vec2, Vec3}, asset::AssetServer};

use crate::{resources::chunks::{Chunks, Chunk}, systems::space::spawn_space::spawn_random_space_background};

pub fn spawn_chunks(
    mut chunks: ResMut<Chunks>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>
){
    //x and y offsets to ensure sprites spawn in middle of screen
   

    for x in 0..3{
        for y in 0..3{
            let x = x as f32;
            let y = y as f32;

            let chunk_id = Vec2::new(x,y);
            let spawn_location = Vec2::new((x * 1920.) - 2880. + 960., (y * 1920.) - 2880. + 960.);
            if !chunks.chunk_exists(chunk_id){
                spawn_random_space_background(&mut commands, &asset_server, Vec3::new(spawn_location.x, spawn_location.y, 0.));
                chunks.chunks.push(Chunk{
                    chunk_id: chunk_id,
                    chunk_location: spawn_location
                });
            }
        }
    }
}