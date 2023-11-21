use bevy::{
    ecs::{
        query::With,
        system::{Commands, Query, ResMut}
    },
    math::{Vec3, Vec2},
    transform::components::Transform,
};

use crate::{
    components::player_starship::PlayerStarship,
    resources::chunks::Chunks,
};

pub fn despawn_chunks(
    player_loc_query: Query<&Transform, With<PlayerStarship>>,
    mut chunks: ResMut<Chunks>,
    mut commands: Commands,
) {
    let mut chunks_to_remove: Vec<Vec2> = Vec::new();

    for chunk in &mut chunks.chunks {
        if !chunk.chunk_visible{ continue; }        

        let chunk_location = Vec3::new(chunk.chunk_location.x, chunk.chunk_location.y, 0.);
        if (player_loc_query.single().translation - chunk_location).length() > 1920. * 3. {
            chunks_to_remove.push(chunk.chunk_id);
            commands.entity(chunk.space_entity).despawn();
            chunk.chunk_visible = false;
        }
    }
}