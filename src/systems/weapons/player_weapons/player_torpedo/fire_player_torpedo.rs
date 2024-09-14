use bevy::{
    ecs::{
        event::EventWriter,
        system::{Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    utils::tracing,
};

use crate::{
    events::combat_events::FirePlayerTorpedoEvent,
    queries::player_starship_queries::PlayerStarshipTransformQuery,
    resources::{
        projectile_ammunition_resource::ProjectileAmmunitionResource,
        selected_weapon::{SelectedWeaponEnum, SelectedWeaponResource},
    },
};

pub fn fire_player_torpedo(
    input: Res<ButtonInput<KeyCode>>,
    ammunition: ResMut<ProjectileAmmunitionResource>,
    weapon_selection: Res<SelectedWeaponResource>,
    player_starships: Query<PlayerStarshipTransformQuery>,
    mut fire_player_torpedo_event: EventWriter<FirePlayerTorpedoEvent>,
) {
    let Ok(_) = player_starships.get_single() else {
        return;
    };

    if weapon_selection.selected_weapon != SelectedWeaponEnum::Torpedo as u32 {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.torpedo_ammunition < 1 {
        tracing::info!("Out of torpedo ammunition");
        return;
    }

    let event = FirePlayerTorpedoEvent {};

    fire_player_torpedo_event.send(event);
}
