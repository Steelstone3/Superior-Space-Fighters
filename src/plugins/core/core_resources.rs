use bevy::app::Plugin;

use crate::resources::{
    camera_settings::CameraSettings, game_state::GameState, sector_size::SectorSize,
};

pub struct CoreResources;

impl Plugin for CoreResources {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CameraSettings {
            zoom_speed: 1.5,
            minimum_zoom: 0.5,
            maximum_zoom: 2.0,
            current_zoom: 1.0,
            zoom_in: 1.1,
            zoom_out: 0.9,
        })
        .insert_resource(GameState { paused: false })
        .insert_resource(SectorSize {
            top_border: 1000.0,
            left_border: -1000.0,
            bottom_border: -1000.0,
            right_border: 1000.0,
        });
    }
}
