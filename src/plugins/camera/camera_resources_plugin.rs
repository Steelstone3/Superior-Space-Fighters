use bevy::app::Plugin;

use crate::resources::camera_settings::CameraSettings;

pub struct CameraResourcesPlugin;

impl Plugin for CameraResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CameraSettings {
            zoom_speed: 1.5,
            minimum_zoom: 0.5,
            maximum_zoom: 2.0,
            current_zoom: 1.0,
            zoom_in: 1.1,
            zoom_out: 0.9,
        });
    }
}
