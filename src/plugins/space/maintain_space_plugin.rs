use bevy::app::{Plugin, Update};

use crate::systems::space::move_empty_space::move_empty_space;

pub struct MaintainSpacePlugin;

impl Plugin for MaintainSpacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, move_empty_space);
    }
}
