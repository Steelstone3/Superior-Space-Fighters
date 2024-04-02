use bevy::{
    ecs::{
        event::EventWriter,
        system::{Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    utils::tracing,
};

use crate::{
    events::game_events::FirePlayerMineEvent,
    queries::player_starship_queries::PlayerStarshipTransformQuery,
    resources::{
        projectile_ammunition::ProjectileAmmunition,
        selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
    },
};

pub fn fire_player_mine(
    input: Res<ButtonInput<KeyCode>>,
    ammunition: ResMut<ProjectileAmmunition>,
    weapon_selection: Res<SelectedWeapon>,
    player_starships: Query<PlayerStarshipTransformQuery>,
    mut fire_player_mine_event: EventWriter<FirePlayerMineEvent>,
) {
    let Ok(_) = player_starships.get_single() else {
        return;
    };

    if weapon_selection.selected_weapon != SelectedWeaponEnum::Mine as u32 {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.mine_ammunition < 1 {
        tracing::info!("Out of mines");
        return;
    }

    let event = FirePlayerMineEvent {};

    fire_player_mine_event.send(event);
}
