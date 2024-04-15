use bevy::app::Plugin;

use crate::resources::{sector_size::SectorSize, targetting_settings::TargettingSettingsResource};

pub struct World;

impl Plugin for World {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SectorSize {
            top_border: 1000.0,
            left_border: -1000.0,
            bottom_border: -1000.0,
            right_border: 1000.0,
        })
        .insert_resource(TargettingSettingsResource {
            maximum_distance: 2000.0,
            is_targetting: false,
        });
    }
}
