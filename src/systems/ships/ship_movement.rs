use crate::{
    components::starship::Starship,
    resources::sector_size::SectorSize,
    systems::controllers::random_generator::{generate_seed, random_value_f32},
};
use bevy::{
    prelude::{Quat, Query, Res, Transform, Vec3},
    time::Time,
};

pub fn ai_movement(
    mut characters: Query<(&mut Transform, &mut Starship)>,
    space_zone_border: Res<SectorSize>,
    time: Res<Time>,
) {
    characters
        .par_iter_mut()
        .for_each(|(mut transform, ai_ship)| {
            let ship_speed = ai_ship.velocity * time.delta_seconds();
            let movement_direction = transform.rotation * Vec3::Y;
            let translation_delta = movement_direction * ship_speed;

            let next_translation = transform.translation + translation_delta;

            if next_translation.y > space_zone_border.top_border
                || next_translation.x < space_zone_border.left_border
                || next_translation.y < space_zone_border.bottom_border
                || next_translation.x > space_zone_border.right_border
            {
                transform.rotate(Quat::from_axis_angle(
                    Vec3::new(0.0, 0.0, 1.0),
                    random_value_f32(generate_seed(), 150.0..210.0),
                ))
            } else {
                transform.translation += translation_delta;
            }
        })
}
