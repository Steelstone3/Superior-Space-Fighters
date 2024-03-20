use bevy::app::Plugin;

use crate::assets::sounds::starships::engines::EngineSound;

pub struct AudioSaveTypesPlugin;

impl Plugin for AudioSaveTypesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<EngineSound>();
    }
}
