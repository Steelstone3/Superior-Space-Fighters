use bevy::app::Plugin;

use crate::resources::{game_state::GameState, sector_size::SectorSize};

pub struct CoreResources;

impl Plugin for CoreResources {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GameState { paused: false })
            .insert_resource(SectorSize {
                top_border: 1000.0,
                left_border: -1000.0,
                bottom_border: -1000.0,
                right_border: 1000.0,
            });
    }
}
