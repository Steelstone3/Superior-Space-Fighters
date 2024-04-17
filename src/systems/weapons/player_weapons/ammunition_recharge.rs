use crate::{
    events::user_interface_events::InGameUserInterfaceEvent,
    resources::{
        projectile_ammunition_recharge::ProjectileAmmunitionRecharge,
        projectile_ammunition_resource::ProjectileAmmunitionResource,
    },
};
use bevy::{
    ecs::event::EventWriter,
    prelude::{Res, ResMut},
    time::Time,
    utils::tracing,
};

pub fn ammunition_recharge(
    time: Res<Time>,
    mut current_ammunition: ResMut<ProjectileAmmunitionResource>,
    mut ammunition_recharge: ResMut<ProjectileAmmunitionRecharge>,
    mut user_interface_event: EventWriter<InGameUserInterfaceEvent>,
) {
    if current_ammunition.blaster_ammunition == current_ammunition.maximum_blaster_ammunition
        && current_ammunition.torpedo_ammunition == current_ammunition.maximum_torpedo_ammunition
        && current_ammunition.mine_ammunition == current_ammunition.maximum_mine_ammunition
        && current_ammunition.exotic_ammunition == current_ammunition.maximum_exotic_ammunition
    {
        return;
    }

    if current_ammunition.blaster_ammunition < current_ammunition.maximum_blaster_ammunition {
        ammunition_recharge.blaster_recovery_rate.tick(time.delta());

        if ammunition_recharge.blaster_recovery_rate.finished() {
            current_ammunition.blaster_ammunition += 1;

            tracing::info!(
                "Recovered 1 blaster. {:?} blaster shots remaining",
                current_ammunition.blaster_ammunition
            );

            user_interface_event.send(InGameUserInterfaceEvent {});
        }
    }

    if current_ammunition.torpedo_ammunition < current_ammunition.maximum_torpedo_ammunition {
        ammunition_recharge.torpedo_recovery_rate.tick(time.delta());

        if ammunition_recharge.torpedo_recovery_rate.finished() {
            current_ammunition.torpedo_ammunition += 1;

            tracing::info!(
                "Recovered 1 torpedo. {:?} torpedoes remaining",
                current_ammunition.torpedo_ammunition
            );

            user_interface_event.send(InGameUserInterfaceEvent {});
        }
    }

    if current_ammunition.mine_ammunition < current_ammunition.maximum_mine_ammunition {
        ammunition_recharge.mine_recovery_rate.tick(time.delta());

        if ammunition_recharge.mine_recovery_rate.finished() {
            current_ammunition.mine_ammunition += 1;

            tracing::info!(
                "Recovered 1 mine. {:?} mines remaining",
                current_ammunition.mine_ammunition
            );

            user_interface_event.send(InGameUserInterfaceEvent {});
        }
    }

    if current_ammunition.exotic_ammunition < current_ammunition.maximum_exotic_ammunition {
        ammunition_recharge.exotic_recovery_rate.tick(time.delta());

        if ammunition_recharge.exotic_recovery_rate.finished() {
            current_ammunition.exotic_ammunition += 1;

            tracing::info!(
                "Recovered 1 exotic. {:?} exotics remaining",
                current_ammunition.exotic_ammunition
            );

            user_interface_event.send(InGameUserInterfaceEvent {});
        }
    }
}
