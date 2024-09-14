use crate::queries::player_mine_queries::MutablePlayerMineEntityQuery;
use bevy::{
    prelude::{Commands, Query, Res},
    time::Time,
    utils::tracing,
};

pub fn player_mine_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut player_mines: Query<MutablePlayerMineEntityQuery>,
) {
    for mut player_mine in &mut player_mines {
        player_mine
            .player_mine
            .mine
            .lifetime_weapon
            .lifetime
            .tick(time.delta());

        if player_mine
            .player_mine
            .mine
            .lifetime_weapon
            .lifetime
            .finished()
        {
            commands.entity(player_mine.entity).despawn();

            tracing::info!("Mine despawn",);
        }
    }
}
