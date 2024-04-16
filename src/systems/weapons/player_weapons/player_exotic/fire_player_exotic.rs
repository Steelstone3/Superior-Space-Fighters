use crate::{
    events::combat_events::FirePlayerExoticEvent,
    queries::player_starship_queries::PlayerStarshipTransformQuery,
    resources::{
        projectile_ammunition_resource::ProjectileAmmunitionResource,
        selected_weapon::{SelectedWeaponEnum, SelectedWeaponResource},
    },
};

use bevy::{ecs::event::EventWriter, input::ButtonInput};
use bevy::{
    prelude::{KeyCode, Query, Res, ResMut},
    utils::tracing,
};

pub fn fire_player_exotic(
    input: Res<ButtonInput<KeyCode>>,
    ammunition: ResMut<ProjectileAmmunitionResource>,
    weapon_selection: Res<SelectedWeaponResource>,
    player_starships: Query<PlayerStarshipTransformQuery>,
    mut fire_player_exotic_event: EventWriter<FirePlayerExoticEvent>,
) {
    let Ok(_) = player_starships.get_single() else {
        return;
    };

    if weapon_selection.selected_weapon != SelectedWeaponEnum::Exotic as u32 {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.exotic_ammunition < 1 {
        tracing::info!("Out of exotic ammunition");
        return;
    }

    let event = FirePlayerExoticEvent {};

    fire_player_exotic_event.send(event);
}
