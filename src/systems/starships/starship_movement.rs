use crate::{
    queries::starship_queries::MutableAIStarshipTransformQuery,
    resources::sector_size::SectorSize,
    systems::controllers::random_generator::{generate_seed, random_value_f32},
};
use bevy::{
    prelude::{Quat, Query, Res, Vec3},
    time::Time,
};

pub fn ai_movement(
    mut starships: Query<MutableAIStarshipTransformQuery>,
    space_zone_border: Res<SectorSize>,
    time: Res<Time>,
) {
    starships.par_iter_mut().for_each(|mut starship| {
        let ship_speed = starship.starship.max_velocity * time.delta_seconds();
        let movement_direction = starship.transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * ship_speed;

        let next_translation = starship.transform.translation + translation_delta;

        if next_translation.y > space_zone_border.top_border
            || next_translation.x < space_zone_border.left_border
            || next_translation.y < space_zone_border.bottom_border
            || next_translation.x > space_zone_border.right_border
        {
            starship.transform.rotate(Quat::from_axis_angle(
                Vec3::new(0.0, 0.0, 1.0),
                random_value_f32(generate_seed(), 150.0..210.0),
            ))
        } else {
            starship.transform.translation += translation_delta;
        }
    })
}
