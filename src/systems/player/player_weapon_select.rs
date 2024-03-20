use crate::resources::weapon_selection::WeaponSelection;
use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Res, ResMut},
    utils::tracing,
};

pub fn player_weapon_select(
    input: Res<ButtonInput<KeyCode>>,
    mut weapon_selection: ResMut<WeaponSelection>,
) {
    // Weapon 1
    if input.just_pressed(KeyCode::Digit1) {
        weapon_selection.selected_weapon = 1;

        tracing::info!("Blaster selected")
    }
    // Weapon 2
    else if input.just_pressed(KeyCode::Digit2) {
        weapon_selection.selected_weapon = 2;

        tracing::info!("Torpedo selected")
    }
    // Weapon 3
    else if input.just_pressed(KeyCode::Digit3) {
        weapon_selection.selected_weapon = 3;

        tracing::info!("Mine selected")
    }
    // Weapon 4
    else if input.just_pressed(KeyCode::Digit4) {
        weapon_selection.selected_weapon = 4;

        tracing::info!("Exotic selected")
    }
}
