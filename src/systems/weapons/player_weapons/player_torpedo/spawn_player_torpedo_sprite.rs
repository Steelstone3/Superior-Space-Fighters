use crate::{
    components::weapons::player_weapons::player_torpedo::PlayerTorpedo,
    events::{combat_events::FirePlayerTorpedoEvent, spawn_sprite_event::SpawnSpriteEvent},
    queries::player_starship_queries::PlayerStarshipTransformQuery,
};
use bevy::{
    ecs::event::{EventReader, EventWriter},
    math::Vec3,
    prelude::{Commands, Query},
};

pub fn spawn_player_torpedo_sprite(
    mut commands: Commands,
    mut fire_player_torpedo_events: EventReader<FirePlayerTorpedoEvent>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    player_starships: Query<PlayerStarshipTransformQuery>,
) {
    let Ok(player_starship) = player_starships.get_single() else {
        return;
    };

    for _ in fire_player_torpedo_events.read() {
        let torpedo_size = 80.0;
        let mut player_transform = *player_starship.transform;
        let torpedo_spawn_position =
            player_transform.translation + player_transform.up() * (torpedo_size / 1.5);
        player_transform.translation = torpedo_spawn_position;
        player_transform.translation.z = 3.0;
        let torpedo = PlayerTorpedo::new(Vec3::new(
            player_transform.translation.x,
            player_transform.translation.y,
            player_transform.translation.z,
        ));

        let image_path = torpedo.torpedo.torpedo.to_string();
        let size = torpedo.torpedo.lock_on_weapon.ranged_weapon.weapon.size;
        let entity = commands.spawn(torpedo).id();

        let event = SpawnSpriteEvent {
            sprite_path: image_path,
            size,
            transform: player_transform,
            entity,
        };

        spawn_sprite_event.send(event);
    }
}
