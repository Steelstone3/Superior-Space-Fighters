use crate::{
    events::{
        combat_events::FirePlayerTorpedoEvent, user_interface_events::InGameUserInterfaceEvent,
    },
    resources::projectile_ammunition_resource::ProjectileAmmunitionResource,
};
use bevy::{
    ecs::event::{EventReader, EventWriter},
    prelude::ResMut,
    utils::tracing,
};

pub fn player_torpedo_ammunition_consumption(
    mut ammunition: ResMut<ProjectileAmmunitionResource>,
    mut fire_player_torpedo_events: EventReader<FirePlayerTorpedoEvent>,
    mut user_interface_event: EventWriter<InGameUserInterfaceEvent>,
) {
    for _ in fire_player_torpedo_events.read() {
        ammunition.torpedo_ammunition -= 1;
        tracing::info!(
            "Fired 1 torpedo. {:?} torpedo ammunition remaining",
            ammunition.torpedo_ammunition
        );

        user_interface_event.send(InGameUserInterfaceEvent {});
    }
}
