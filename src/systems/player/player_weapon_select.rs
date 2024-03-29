use crate::{
    events::user_interface_event::UserInterfaceEvent,
    resources::selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
};
use bevy::{
    ecs::event::EventWriter,
    input::ButtonInput,
    prelude::{KeyCode, Res, ResMut},
    utils::tracing,
};

pub fn player_weapon_select(
    input: Res<ButtonInput<KeyCode>>,
    mut weapon_selection: ResMut<SelectedWeapon>,
    mut selected_weapon_event: EventWriter<UserInterfaceEvent>,
) {
    // Weapon 1
    if input.just_pressed(KeyCode::Digit1) {
        weapon_selection.selected_weapon = SelectedWeaponEnum::Blaster as u32;
        selected_weapon_event.send(UserInterfaceEvent);
        tracing::info!("Blaster selected")
    }
    // Weapon 2
    else if input.just_pressed(KeyCode::Digit2) {
        weapon_selection.selected_weapon = SelectedWeaponEnum::Torpedo as u32;
        selected_weapon_event.send(UserInterfaceEvent);
        tracing::info!("Torpedo selected")
    }
    // Weapon 3
    else if input.just_pressed(KeyCode::Digit3) {
        weapon_selection.selected_weapon = SelectedWeaponEnum::Mine as u32;
        selected_weapon_event.send(UserInterfaceEvent);
        tracing::info!("Mine selected")
    }
    // Weapon 4
    else if input.just_pressed(KeyCode::Digit4) {
        weapon_selection.selected_weapon = SelectedWeaponEnum::Exotic as u32;
        selected_weapon_event.send(UserInterfaceEvent);
        tracing::info!("Exotic selected")
    }
}
