use crate::{
    components::{space::Space, starships::player_starship::PlayerStarship},
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
    let number_of_tiles = 5;
    let total_area_around_player = 1920.0 * 2.5;
    let half_space_tile_size = 1920.0 * 0.5;

    //use player location to determine what chunks to load
    let player_translation = player_translation_query.single().translation;
    let space_tile_size = 1920.0;

    let mut x_start = 0;
    let mut y_start = 0;

    if player_translation.x > 0.0 {
        x_start = ((player_translation.x - (half_space_tile_size * 2.0)) / space_tile_size)
            .ceil()
            .clamp(sector_size.left_border, sector_size.right_border) as i32;
    } else if player_translation.x < 0.0 {
        x_start = ((player_translation.x + (half_space_tile_size * 2.0)) / space_tile_size)
            .floor()
            .clamp(sector_size.left_border, sector_size.right_border) as i32;
    }

    if player_translation.y > 0.0 {
        y_start = ((player_translation.y - (half_space_tile_size * 2.0)) / space_tile_size)
            .ceil()
            .clamp(sector_size.bottom_border, sector_size.top_border) as i32;
    } else if player_translation.y < 0.0 {
        y_start = ((player_translation.y + (half_space_tile_size * 2.0)) / space_tile_size)
            .floor()
            .clamp(sector_size.bottom_border, sector_size.top_border) as i32;
    }

    let mut space_enumeration = space_query.iter_mut();
    for x in x_start..x_start + number_of_tiles {
        for y in y_start..y_start + number_of_tiles {
            if let Some(mut space_item) = space_enumeration.next() {
                space_item.0.translation = Vec3::new(
                    (x as f32 * space_tile_size) - total_area_around_player + half_space_tile_size,
                    (y as f32 * space_tile_size) - total_area_around_player + half_space_tile_size,
                    0.0,
                );
            }
        }
    }
}
