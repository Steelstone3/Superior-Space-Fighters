use bevy::{
    ecs::{
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query},
    },
    transform::components::Transform,
};

use crate::components::{
    player_starship::PlayerStarship, starship::Starship, weapons::target::Target,
};

pub fn update_player_targeting(
    player_transform: Query<&Transform, (With<PlayerStarship>, Without<Starship>)>,
    mut player_target: Query<
        (&mut Transform, &mut Target, Entity),
        (Without<Starship>, Without<PlayerStarship>),
    >,
    other_starships: Query<&mut Transform, With<Starship>>,
    mut commands: Commands,
) {
    player_target
        .iter_mut()
        .for_each(|(mut transform, target, entity)| {
            //Ensure target is valid
            let Ok(target_ship) = other_starships.get(target.target_entity) else {
                return;
            };

            let Ok(player_transform) = player_transform.get_single() else {
                return;
            };

            if (player_transform.translation - target_ship.translation).length() < 500.0 {
                transform.translation = target_ship.translation;
                transform.rotation = target_ship.rotation;
            } else {
                commands.entity(entity).despawn();
            }
        });
}
