use bevy::app::Plugin;

use crate::resources::targetting_settings::TargettingSettingsResource;

pub struct UserInterfaceTargetingResourcesPlugin;

impl Plugin for UserInterfaceTargetingResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TargettingSettingsResource {
            maximum_distance: 2000.0,
            is_targetting: false,
        });
    }
}
