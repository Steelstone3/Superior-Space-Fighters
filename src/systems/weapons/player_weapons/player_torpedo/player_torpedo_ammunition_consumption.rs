use crate::{
    events::combat_events::FirePlayerTorpedoEvent,
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{ecs::event::EventReader, prelude::ResMut, utils::tracing};

pub fn player_torpedo_ammunition_consumption(
    mut ammunition: ResMut<ProjectileAmmunition>,
    mut fire_player_torpedo_events: EventReader<FirePlayerTorpedoEvent>,
) {
    for _ in fire_player_torpedo_events.read() {
        ammunition.torpedo_ammunition -= 1;
        tracing::info!(
            "Fired 1 torpedo. {:?} torpedo ammunition remaining",
            ammunition.torpedo_ammunition
        );
    }
}
