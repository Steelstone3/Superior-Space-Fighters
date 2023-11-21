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
    resources::{
        chunks::{Chunk, Chunks},
        sector_size::SectorSize,
    },
    systems::space::spawn_space::spawn_random_space_background,
};

pub fn spawn_chunks(
    player_tran_query: Query<&Transform, With<PlayerStarship>>,
    mut chunks: ResMut<Chunks>,
    mut commands: Commands,
    sector_size: Res<SectorSize>,
    asset_server: Res<AssetServer>,
) {
    //use player location to determine what chuncks to load
    let player_tran = player_tran_query.single().translation;
    let mut x_start = 0;
    let mut y_start = 0;

    if player_tran.x > 0. {
        x_start = (player_tran.x / 1920.)
            .ceil()
            .clamp(sector_size.left_border, sector_size.right_border) as i32;
    } else if player_tran.x < 0. {
        x_start = (player_tran.x / 1920.)
            .floor()
            .clamp(sector_size.left_border, sector_size.right_border) as i32;
    }

    if player_tran.y > 0. {
        y_start = (player_tran.y / 1920.)
            .ceil()
            .clamp(sector_size.bottom_border, sector_size.top_border) as i32;
    } else if player_tran.y < 0. {
        y_start = (player_tran.y / 1920.)
            .floor()
            .clamp(sector_size.bottom_border, sector_size.top_border) as i32;
    }

    for x in x_start..x_start + 3 {
        for y in y_start..y_start + 3 {
            let x = x as f32;
            let y = y as f32;

            let chunk_id = Vec2::new(x, y);
            let spawn_location = Vec2::new((x * 1920.) - 2880. + 960., (y * 1920.) - 2880. + 960.);
            if !chunks.chunk_exists(chunk_id) {
                let space_entity = spawn_random_space_background(
                    &mut commands,
                    &asset_server,
                    Vec3::new(spawn_location.x, spawn_location.y, 0.),
                );
                chunks.chunks.push(Chunk {
                    chunk_id,
                    chunk_location: spawn_location,
                    chunk_visible: true,
                    space_entity
                });
            }else if !chunks.get_visibility(chunk_id){
                chunks.set_chunk_visibility(chunk_id, true);
                let space_entity = spawn_random_space_background( &mut commands,
                    &asset_server,
                    Vec3::new(spawn_location.x, spawn_location.y, 0.),);
                chunks.set_space_entity(chunk_id, space_entity)
            }
        }
    }
}
