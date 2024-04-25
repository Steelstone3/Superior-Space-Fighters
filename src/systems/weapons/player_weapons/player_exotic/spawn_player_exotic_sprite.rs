use crate::{
    components::weapons::player_weapons::player_exotic::PlayerExotic,
    events::spawn_sprite_event::SpawnSpriteEvent,
    queries::player_starship_queries::PlayerStarshipTransformQuery,
};
use bevy::ecs::event::EventWriter;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Query};

pub fn spawn_player_exotic_sprite(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    player_starships: Query<PlayerStarshipTransformQuery>,
) {
    let Ok(player_starship) = player_starships.get_single() else {
        return;
    };

    let mut player_transform = *player_starship.transform;
    player_transform.translation.z = 3.0;
    let exotic = PlayerExotic::new(Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        player_transform.translation.z,
    ));

    let image_path = exotic.exotic.exotic.to_string();
    let size = exotic.exotic.ranged_weapon.weapon.size;
    let entity = commands.spawn(exotic).id();

    let event = SpawnSpriteEvent {
        sprite_path: image_path,
        size,
        transform: player_transform,
        entity,
    };

    spawn_sprite_event.send(event);
}
