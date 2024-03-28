use bevy::{ecs::event::EventWriter, prelude::Vec3};

use crate::{
    components::starships::player_starship::PlayerStarship,
    events::spawn_player_sprite_event::SpawnPlayerSpriteEvent,
};

pub fn spawn_player_ship(mut ev_spawn_sprite: EventWriter<SpawnPlayerSpriteEvent>) {
    let player = PlayerStarship::default();
    let texture = player.ship.faction_starship.to_string();

    let sprite_event = SpawnPlayerSpriteEvent {
        sprite_path: texture,
        size: player.ship.size,
        translation: Vec3::new(0.0, 0.0, 4.0),
        player,
    };

    ev_spawn_sprite.send(sprite_event);
}
