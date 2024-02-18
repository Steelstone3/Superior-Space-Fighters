use bevy::{
    ecs::{
        query::{With, Without},
        system::Query,
    },
    transform::components::Transform,
};

use crate::components::{starship::Starship, target::Target};

pub fn update_player_targeting(
    mut player_target: Query<(&mut Transform, &mut Target), Without<Starship>>,
    entities: Query<&mut Transform, With<Starship>>,
) {
    player_target
        .iter_mut()
        .for_each(|(mut transform, target)| {
            let Ok(target_ship) = entities.get(target.target_entity) else {
                return;
            };
            transform.translation = target_ship.translation;
            transform.rotation = target_ship.rotation;
        });
}
