use crate::{
    events::user_interface_events::InGameUserInterfaceEvent,
    resources::projectile_ammunition_resource::ProjectileAmmunitionResource,
};
use bevy::{ecs::event::EventWriter, prelude::ResMut, utils::tracing};

pub fn player_torpedo_ammunition_consumption(
    mut ammunition: ResMut<ProjectileAmmunitionResource>,
    mut user_interface_event: EventWriter<InGameUserInterfaceEvent>,
) {
    ammunition.torpedo_ammunition -= 1;
    tracing::info!(
        "Fired 1 torpedo. {:?} torpedo ammunition remaining",
        ammunition.torpedo_ammunition
    );

    user_interface_event.send(InGameUserInterfaceEvent {});
}
