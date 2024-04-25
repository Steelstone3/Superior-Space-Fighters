use crate::{
    events::user_interface_events::InGameUserInterfaceEvent,
    resources::projectile_ammunition_resource::ProjectileAmmunitionResource,
};
use bevy::{ecs::event::EventWriter, prelude::ResMut, utils::tracing};

pub fn player_mine_ammunition_consumption(
    mut ammunition: ResMut<ProjectileAmmunitionResource>,
    mut user_interface_event: EventWriter<InGameUserInterfaceEvent>,
) {
    ammunition.mine_ammunition -= 1;
    tracing::info!(
        "Fired 1 mine. {:?} mines remaining",
        ammunition.mine_ammunition
    );

    user_interface_event.send(InGameUserInterfaceEvent {});
}
