use crate::components::{player_mine::PlayerMine, starship::Starship};
use bevy::{
    ecs::query::Without,
    prelude::{Commands, Entity, Query},
    transform::components::Transform,
    utils::tracing,
};

// TODO multi-thread
pub fn player_mine_collision_with_starship(
    mut commands: Commands,
    mut mines: Query<(Entity, &mut Transform, &mut PlayerMine), Without<Starship>>,
    mut starships: Query<(Entity, &mut Transform, &mut Starship)>,
) {
    for (mine_entity, mine_transform, _mine) in &mut mines {
        for (starship_entity, starship_transform, starship) in &mut starships {
            let distance_to_starship =
                (mine_transform.translation - starship_transform.translation).length();

            let is_collision =
                distance_to_starship < starship.size.x || distance_to_starship < starship.size.y;

            if is_collision {
                tracing::info!("Mine collision with starship");

                commands.entity(mine_entity).despawn();
                commands.entity(starship_entity).despawn();
            }
        }
    }
}
