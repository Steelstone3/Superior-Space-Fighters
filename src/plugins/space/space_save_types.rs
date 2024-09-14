use bevy::app::Plugin;

use crate::{
    assets::images::{space::SpaceSprite, space_stations::SpaceStationSprite},
    components::{space::Space, station::SpaceStation},
};

pub struct SpaceSaveTypesPlugin;

impl Plugin for SpaceSaveTypesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Space>();
        app.register_type::<SpaceSprite>();
        app.register_type::<SpaceStation>();
        app.register_type::<SpaceStationSprite>();
    }
}
