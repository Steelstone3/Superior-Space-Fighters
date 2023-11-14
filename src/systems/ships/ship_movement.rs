use bevy::{
    prelude::{Quat, Query, Res, Transform, Vec3},
    time::Time,
};

use crate::{
    components::starship::Starship,
    resources::space_zone_border::SpaceZoneBorder,
    systems::controllers::random_generator::{generate_seed, random_range_f32},
};

pub fn ai_movement(
    mut characters: Query<(&mut Transform, &mut Starship)>,
    space_zone_border: Res<SpaceZoneBorder>,
    time: Res<Time>,
) {
    characters.par_iter_mut().for_each(|(mut transform, mut ai_ship)| {
        let ship_speed = ai_ship.current_velocity * time.delta_seconds();

        ai_ship.current_velocity = (ai_ship.current_velocity + ai_ship.acceleration)
            .clamp(-ai_ship.velocity, ai_ship.velocity);

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
                random_range_f32(generate_seed(), 150.0, 210.0),
            ))
        } else {
            transform.translation += translation_delta;
        }
    })
}
