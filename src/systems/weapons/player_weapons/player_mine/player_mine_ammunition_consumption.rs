use crate::{
    events::game_events::FirePlayerMineEvent,
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{ecs::event::EventReader, prelude::ResMut, utils::tracing};

pub fn player_mine_ammunition_consumption(
    mut ammunition: ResMut<ProjectileAmmunition>,
    mut fire_player_mine_events: EventReader<FirePlayerMineEvent>,
) {
    for _ in fire_player_mine_events.read() {
        ammunition.mine_ammunition -= 1;
        tracing::info!(
            "Fired 1 mine. {:?} mines remaining",
            ammunition.mine_ammunition
        );
    }
}
