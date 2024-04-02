use crate::{
    components::weapons::player_weapons::player_blaster::PlayerBlaster,
    events::{game_events::FirePlayerBlasterEvent, spawn_sprite_event::SpawnSpriteEvent},
    queries::player_starship_queries::PlayerStarshipTransformQuery,
};
use bevy::{
    ecs::event::{EventReader, EventWriter},
    math::Vec3,
    prelude::{Commands, Query},
};

pub fn spawn_player_blaster_sprite(
    mut commands: Commands,
    mut fire_player_blaster_event: EventReader<FirePlayerBlasterEvent>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    player_starships: Query<PlayerStarshipTransformQuery>,
) {
    let Ok(player_starship) = player_starships.get_single() else {
        return;
    };

    for _ in fire_player_blaster_event.read() {
        let mut player_transform = *player_starship.transform;
        player_transform.translation.z = 3.0;

        let blaster = PlayerBlaster::new(Vec3::new(
            player_transform.translation.x,
            player_transform.translation.y,
            player_transform.translation.z,
        ));

        let image_path = blaster.blaster.blaster.to_string();
        let size = blaster.blaster.ranged_weapon.weapon.size;
        let entity = commands.spawn(blaster).id();

        let event = SpawnSpriteEvent {
            sprite_path: image_path,
            size,
            transform: player_transform,
            entity,
        };

        spawn_sprite_event.send(event);
    }
}
