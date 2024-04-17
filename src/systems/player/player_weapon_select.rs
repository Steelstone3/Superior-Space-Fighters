use crate::{
    events::user_interface_events::InGameUserInterfaceEvent,
    resources::selected_weapon::{SelectedWeaponEnum, SelectedWeaponResource},
};
use bevy::{
    ecs::event::EventWriter,
    input::ButtonInput,
    prelude::{KeyCode, Res, ResMut},
    utils::tracing,
};

pub fn player_weapon_select(
    input: Res<ButtonInput<KeyCode>>,
    mut weapon_selection: ResMut<SelectedWeaponResource>,
    mut user_interface_event: EventWriter<InGameUserInterfaceEvent>,
) {
    // Weapon 1
    if input.just_pressed(KeyCode::Digit1) {
        weapon_selection.selected_weapon = SelectedWeaponEnum::Blaster as u32;
        user_interface_event.send(InGameUserInterfaceEvent);
        tracing::info!("Blaster selected")
    }
    // Weapon 2
    else if input.just_pressed(KeyCode::Digit2) {
        weapon_selection.selected_weapon = SelectedWeaponEnum::Torpedo as u32;
        user_interface_event.send(InGameUserInterfaceEvent);
        tracing::info!("Torpedo selected")
    }
    // Weapon 3
    else if input.just_pressed(KeyCode::Digit3) {
        weapon_selection.selected_weapon = SelectedWeaponEnum::Mine as u32;
        user_interface_event.send(InGameUserInterfaceEvent);
        tracing::info!("Mine selected")
    }
    // Weapon 4
    else if input.just_pressed(KeyCode::Digit4) {
        weapon_selection.selected_weapon = SelectedWeaponEnum::Exotic as u32;
        user_interface_event.send(InGameUserInterfaceEvent);
        tracing::info!("Exotic selected")
    }
}
