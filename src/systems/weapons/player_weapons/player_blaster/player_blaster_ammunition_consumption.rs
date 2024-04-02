use crate::{
    events::game_events::FirePlayerBlasterEvent,
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{ecs::event::EventReader, prelude::ResMut, utils::tracing};

pub fn player_blaster_ammunition_consumption(
    mut ammunition: ResMut<ProjectileAmmunition>,
    mut fire_player_blaster_events: EventReader<FirePlayerBlasterEvent>,
) {
    for _ in fire_player_blaster_events.read() {
        ammunition.blaster_ammunition -= 1;
        tracing::info!(
            "Fired 1 blaster shot. {:?} blaster shots remaining",
            ammunition.blaster_ammunition
        );
    }
}
