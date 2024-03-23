use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Quat,
    prelude::Vec3,
};

use crate::{
    components::starships::player_starship::PlayerStarship,
    events::spawn_sprite_event::SpawnSpriteEvent,
};

pub fn spawn_player_ship(
    mut ev_spawn_sprite: EventWriter<SpawnSpriteEvent>,
    mut commands: Commands,
) {
    let player = PlayerStarship::default();
    let texture = player.ship.faction_starship.to_string();
    let size = player.ship.size;

    let entity = commands.spawn(player).id();

    let sprite_event = SpawnSpriteEvent {
        sprite_path: texture,
        size,
        translation: Vec3::new(0.0, 0.0, 4.0),
        entity,
        rotation: Quat::default(),
    };
    ev_spawn_sprite.send(sprite_event);
}
