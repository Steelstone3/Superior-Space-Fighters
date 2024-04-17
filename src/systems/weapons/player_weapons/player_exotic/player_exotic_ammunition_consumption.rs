use crate::{
    events::{
        combat_events::FirePlayerExoticEvent, user_interface_events::InGameUserInterfaceEvent,
    },
    resources::projectile_ammunition_resource::ProjectileAmmunitionResource,
};
use bevy::{
    ecs::event::{EventReader, EventWriter},
    prelude::ResMut,
    utils::tracing,
};

pub fn player_exotic_ammunition_consumption(
    mut ammunition: ResMut<ProjectileAmmunitionResource>,
    mut fire_player_exotic_events: EventReader<FirePlayerExoticEvent>,
    mut user_interface_event: EventWriter<InGameUserInterfaceEvent>,
) {
    for _ in fire_player_exotic_events.read() {
        ammunition.exotic_ammunition -= 1;
        tracing::info!(
            "Fired 1 exotic shot. {:?} exotic shots remaining",
            ammunition.exotic_ammunition
        );

        user_interface_event.send(InGameUserInterfaceEvent {});
    }
}
