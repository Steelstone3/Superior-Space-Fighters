use bevy::{ecs::{system::{ResMut, Commands, Res, Query}, query::With}, math::{Vec2, Vec3}, asset::AssetServer, transform::components::Transform};

use crate::{resources::chunks::{Chunks, Chunk}, systems::space::spawn_space::spawn_random_space_background, components::player_starship::PlayerStarship};

pub fn spawn_chunks(
    player_tran_query: Query<&Transform, With<PlayerStarship>>,
    mut chunks: ResMut<Chunks>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>
){
    //TODO replace hardcoded numbers with values from ChunkSettingsResource

    //use player location to determine what chuncks to load
    let player_tran = player_tran_query.single().translation;
    let mut x_start = 0;
    let mut y_start = 0;

    if  player_tran.x > 0. {
        x_start = (player_tran.x / 1920.).ceil() as i32;
    }else if player_tran.x < 0. {
        x_start = (player_tran.x / 1920.).floor() as i32;
    }

    if player_tran.y > 0.{
        y_start = (player_tran.y / 1920.).ceil() as i32;
    }else if player_tran.y < 0.{
        y_start = (player_tran.y / 1920.).floor() as i32;
    }

    for x in x_start..x_start + 3{
        for y in y_start..y_start + 3{
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