use bevy::app::Plugin;

use crate::events::ui_selected_weapon_event::UISelectedWeaponEvent;

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UISelectedWeaponEvent>();
    }
}
