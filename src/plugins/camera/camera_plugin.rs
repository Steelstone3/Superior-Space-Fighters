use bevy::app::Plugin;

use super::{
    camera_resources_plugin::CameraResourcesPlugin, camera_startup_plugin::CameraStartupPlugin,
    camera_update_plugin::CameraUpdatePlugin,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            CameraUpdatePlugin,
            CameraStartupPlugin,
            CameraResourcesPlugin,
        ));
    }
}
