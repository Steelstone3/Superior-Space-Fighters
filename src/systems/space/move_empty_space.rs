use crate::{
    components::{player_starship::PlayerStarship, space::Space},
    resources::sector_size::SectorSize,
};
use bevy::{
    ecs::{
        query::{With, Without},
        system::{Query, Res},
    },
    math::Vec3,
    transform::components::Transform,
};

pub fn move_empty_space(
    player_translation_query: Query<&Transform, With<PlayerStarship>>,
    sector_size: Res<SectorSize>,
    mut space_query: Query<(&mut Transform, &mut Space), Without<PlayerStarship>>,
) {
    let number_of_tiles = 3;
    let total_area_around_player = 1920.0 * 1.5;
    let half_space_tile_size = 1920.0 * 0.5;

    //use player location to determine what chunks to load
    let player_translation = player_translation_query.single().translation;
    let space_tile_size = 1920.0;

    let x_start = ((player_translation.x - half_space_tile_size) / space_tile_size)
        .ceil()
        .clamp(sector_size.left_border, sector_size.right_border) as i32;

    let y_start = ((player_translation.y - half_space_tile_size) / space_tile_size)
        .ceil()
        .clamp(sector_size.bottom_border, sector_size.top_border) as i32;

    let mut space_enumeration = space_query.iter_mut().enumerate();
    let mut used_indexs: [i32; 9] = [-1, -1, -1, -1, -1, -1, -1, -1, -1];
    for x in x_start..x_start + number_of_tiles {
        for y in y_start..y_start + number_of_tiles {
            for (i, mut space_item) in &mut space_enumeration {
                let index_int = i as i32;
                if !used_indexs.contains(&index_int) {
                    space_item.0.translation = Vec3::new(
                        (x as f32 * space_tile_size) - total_area_around_player
                            + half_space_tile_size,
                        (y as f32 * space_tile_size) - total_area_around_player
                            + half_space_tile_size,
                        0.0,
                    );
                    used_indexs[i] = index_int;
                    break;
                }
            }
        }
    }
}
