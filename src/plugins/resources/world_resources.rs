use bevy::app::Plugin;

use crate::resources::{sector_size::SectorSize, targetting_settings::TargettingSettings};

pub struct WorldResources;

impl Plugin for WorldResources {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SectorSize {
            top_border: 1000.0,
            left_border: -1000.0,
            bottom_border: -1000.0,
            right_border: 1000.0,
        })
        .insert_resource(TargettingSettings {
            maximum_distance: 2000.0,
            is_targetting: false,
        });
    }
}
