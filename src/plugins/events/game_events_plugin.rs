use bevy::app::Plugin;

use crate::events::{
    spawn_planet_sprite_event::SpawnPlanetSpriteEvent,
    spawn_player_sprite_event::SpawnPlayerSpriteEvent,
    spawn_starship_sprite_event::SpawnStarshipSpriteEvent,
    ui_selected_weapon_event::UISelectedWeaponEvent,
};

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UISelectedWeaponEvent>();
        app.add_event::<SpawnPlayerSpriteEvent>();
        app.add_event::<SpawnPlanetSpriteEvent>();
        app.add_event::<SpawnStarshipSpriteEvent>();
    }
}
