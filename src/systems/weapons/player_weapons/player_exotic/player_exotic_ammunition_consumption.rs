use crate::{
    events::user_interface_events::InGameUserInterfaceEvent,
    resources::projectile_ammunition_resource::ProjectileAmmunitionResource,
};
use bevy::{ecs::event::EventWriter, prelude::ResMut, utils::tracing};

pub fn player_exotic_ammunition_consumption(
    mut ammunition: ResMut<ProjectileAmmunitionResource>,
    mut user_interface_event: EventWriter<InGameUserInterfaceEvent>,
) {
    ammunition.exotic_ammunition -= 1;
    tracing::info!(
        "Fired 1 exotic shot. {:?} exotic shots remaining",
        ammunition.exotic_ammunition
    );

    user_interface_event.send(InGameUserInterfaceEvent {});
}
