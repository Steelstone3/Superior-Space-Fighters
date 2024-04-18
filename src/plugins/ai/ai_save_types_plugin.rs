use bevy::app::Plugin;

use crate::{
    assets::images::starships::faction_starships::FactionStarshipSprite,
    components::starships::{hull::Hull, shield::Shield, starship::Starship},
};

pub struct AISaveTypesPlugin;

impl Plugin for AISaveTypesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Starship>();
        app.register_type::<Shield>();
        app.register_type::<Hull>();
        app.register_type::<FactionStarshipSprite>();
    }
}
