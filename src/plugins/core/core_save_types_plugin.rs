use bevy::{app::Plugin, transform::components::Transform};

pub struct CoreSaveTypesPlugin;

impl Plugin for CoreSaveTypesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Transform>();
    }
}
