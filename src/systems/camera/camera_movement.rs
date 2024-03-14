use crate::components::starships::player_starship::PlayerStarship;
use bevy::{
    ecs::{
        query::{With, Without},
        system::Query,
    },
    render::camera::Camera,
    transform::components::Transform,
};

pub fn camera_movement(
    player: Query<&Transform, With<PlayerStarship>>,
    mut camera: Query<(&mut Transform, &mut Camera), Without<PlayerStarship>>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    let Ok((mut camera_transform, _)) = camera.get_single_mut() else {
        return;
    };

    camera_transform.translation = player.translation;
}
