use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_resource_available,
    resources::selected_weapon::SelectedWeaponResource,
    systems::user_interface::{
        draw_user_interface::draw_user_interface,
        weapon_selection_update::update_weapon_selection_icons,
    },
};

pub struct UserInterfaceWeaponSelectionPlugin;

impl Plugin for UserInterfaceWeaponSelectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            update_weapon_selection_icons
                .run_if(run_if_resource_available::<SelectedWeaponResource>),
        )
        .add_systems(Startup, draw_user_interface);
    }
}
