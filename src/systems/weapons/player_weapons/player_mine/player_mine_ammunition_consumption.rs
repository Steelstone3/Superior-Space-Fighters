use crate::{
    events::{combat_events::FirePlayerMineEvent, user_interface_events::InGameUserInterfaceEvent},
    resources::projectile_ammunition_resource::ProjectileAmmunitionResource,
};
use bevy::{
    ecs::event::{EventReader, EventWriter},
    prelude::ResMut,
    utils::tracing,
};

pub fn player_mine_ammunition_consumption(
    mut ammunition: ResMut<ProjectileAmmunitionResource>,
    mut fire_player_mine_events: EventReader<FirePlayerMineEvent>,
    mut user_interface_event: EventWriter<InGameUserInterfaceEvent>,
) {
    for _ in fire_player_mine_events.read() {
        ammunition.mine_ammunition -= 1;
        tracing::info!(
            "Fired 1 mine. {:?} mines remaining",
            ammunition.mine_ammunition
        );

        user_interface_event.send(InGameUserInterfaceEvent {});
    }
}
